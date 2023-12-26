use std::{collections::HashMap, sync::Arc};

use log::error;
use tokio::sync::{
    watch::{Receiver, Sender},
    Mutex,
};
use uuid::Uuid;

type WatchHashMap<I, T> = HashMap<Uuid, (Option<I>, Sender<T>)>;

/// Clone is cheap as everything is behind an `Arc`
#[derive(Clone)]
pub struct WatcherController<T, I> {
    watchers: Arc<Mutex<WatchHashMap<I, T>>>,
}

impl<T: Clone + Default, I: PartialEq> WatcherController<T, I> {
    pub fn new() -> Self {
        Self {
            watchers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_watcher(&self, client_id: Option<I>) -> Receiver<T> {
        let (tx, rx) = tokio::sync::watch::channel(T::default());
        let uuid = Uuid::new_v4();
        {
            let mut lock = self.watchers.lock().await;
            (*lock).insert(uuid, (client_id, tx));
        }
        log::info!("Watcher {uuid} added");
        rx
    }

    async fn remove_watcher(&self, uuid: &Uuid) {
        let mut lock = self.watchers.lock().await;
        (*lock).remove_entry(uuid);
    }

    pub async fn notify(&self, value: T, client_id: Option<I>) {
        let mut uuid_to_remove = vec![];
        for (uuid, (watcher_client_id, sender)) in &*self.watchers.lock().await {
            if sender.is_closed() {
                error!("Channel {uuid} has been closed...");
                uuid_to_remove.push(*uuid);
                continue;
            }

            // Only notifying is no client_id is specified or if we are handling asked client_id
            if watcher_client_id.is_none() || *watcher_client_id == client_id {
                log::debug!("Notifying watcher {uuid}...");
                if let Err(e) = sender.send(value.clone()) {
                    error!("Error while notifying watcher {uuid}: {e}");
                    uuid_to_remove.push(*uuid);
                    continue;
                }
            }
        }

        for uuid in uuid_to_remove {
            self.remove_watcher(&uuid).await;
        }
    }
}
