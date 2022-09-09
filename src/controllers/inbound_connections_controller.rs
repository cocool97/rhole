use std::{collections::HashSet, net::SocketAddr, ops::Deref, sync::Arc};

use anyhow::Result;
use tokio::net::UdpSocket;

use crate::models::NetConfig;

#[derive(Clone)]
pub struct InboundConnectionsController {
    config: NetConfig,
    proxy_server: String,
    blacklist: Arc<HashSet<String>>,
}

impl InboundConnectionsController {
    pub fn new(config: NetConfig, proxy_server: String, blacklist: HashSet<String>) -> Self {
        Self {
            config,
            proxy_server,
            blacklist: Arc::new(blacklist),
        }
    }

    pub async fn listen(self) -> Result<()> {
        let socket = Arc::new(
            UdpSocket::bind((self.config.listen_addr.as_str(), self.config.listen_port)).await?,
        );
        loop {
            let mut buffer = [0u8; 4096];

            // TODO: handle errors here
            let (number_of_bytes, origin_addr) = socket.recv_from(&mut buffer).await?;
            log::debug!("Received connection from {}", origin_addr);

            let a = self.blacklist.clone();
            let proxy = self.proxy_server.clone();
            let j = socket.clone();

            tokio::spawn(async move {
                if let Err(e) = Self::handle_packet(
                    j.deref(),
                    buffer,
                    a.deref(),
                    origin_addr,
                    number_of_bytes,
                    proxy,
                )
                .await
                {
                    println!("{}", e);
                }
            });
        }
    }

    async fn handle_packet(
        socket: &UdpSocket,
        buffer: [u8; 4096],
        nogoes: &HashSet<String>,
        origin_addr: SocketAddr,
        number_of_bytes: usize,
        proxy: String,
    ) -> Result<()> {
        let packet = dns_parser::Packet::parse(&buffer[..number_of_bytes])?;

        if nogoes.contains(packet.questions.first().unwrap().qname.to_string().as_str()) {
            log::warn!("NO GO");
            // TODO: Return precompiled DNS response
        } else {
            let proxy_socket = UdpSocket::bind("0.0.0.0:0").await?;
            proxy_socket
                .send_to(&buffer[..number_of_bytes], proxy)
                .await?;
            let mut res_buffer = [0u8; 4096];
            let received_size = proxy_socket.recv(&mut res_buffer).await?;
            socket
                .send_to(&res_buffer[..received_size], origin_addr)
                .await?;
        }

        Ok(())
    }
}
