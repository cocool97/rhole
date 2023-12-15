use std::{collections::HashMap, sync::Arc};

use log::error;
use tokio::sync::{
    watch::{Receiver, Sender},
    Mutex,
};
use uuid::Uuid;

/// Clone is cheap as everything is behind an `Arc`
#[derive(Clone)]
pub struct WatcherController<T> {
    watchers: Arc<Mutex<HashMap<Uuid, Sender<T>>>>,
}

impl<T: Clone + Default> WatcherController<T> {
    pub fn new() -> Self {
        Self {
            watchers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_watcher(&self) -> Receiver<T> {
        let (tx, rx) = tokio::sync::watch::channel(T::default());
        let uuid = Uuid::new_v4();
        {
            let mut lock = self.watchers.lock().await;
            (*lock).insert(uuid, tx);
        }
        log::info!("Watcher {uuid} added");
        rx
    }

    async fn remove_watcher(&self, uuid: &Uuid) {
        log::debug!("Watcher {uuid} exited...");
        {
            let mut lock = self.watchers.lock().await;
            (*lock).remove_entry(uuid);
        }
    }

    pub async fn notify(&self, value: T) {
        let mut uuid_to_remove = vec![];
        for (uuid, sender) in &*self.watchers.lock().await {
            log::debug!("Notifying watcher {uuid}...");
            if sender.is_closed() {
                error!("Channel {uuid} has been closed...");
                uuid_to_remove.push(*uuid);
                continue;
            }
            if let Err(e) = sender.send(value.clone()) {
                error!("Error while notifying watcher {uuid}: {e}");
                uuid_to_remove.push(*uuid);
                continue;
            }
        }

        for uuid in uuid_to_remove {
            self.remove_watcher(&uuid).await;
        }
    }
}
