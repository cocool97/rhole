use std::{collections::HashSet, net::SocketAddr, ops::Deref, sync::Arc};

use anyhow::Result;
use tokio::net::UdpSocket;

#[derive(Clone)]
pub struct InboundConnectionsController {
    proxy_server: Arc<String>,
    blacklist: Arc<HashSet<String>>,
}

impl InboundConnectionsController {
    pub fn new(proxy_server: String, blacklist: HashSet<String>) -> Self {
        Self {
            proxy_server: Arc::new(proxy_server),
            blacklist: Arc::new(blacklist),
        }
    }

    pub async fn listen(self, addr: &str, port: u16) -> Result<()> {
        let socket = Arc::new(UdpSocket::bind((addr, port)).await?);
        loop {
            let mut buffer = [0u8; 4096];

            // TODO: handle errors here
            let (number_of_bytes, origin_addr) = socket.recv_from(&mut buffer).await?;
            log::debug!("Received connection from {}", origin_addr);

            let blacklist = self.blacklist.clone();
            let proxy = self.proxy_server.clone();
            let core_socket = socket.clone();

            tokio::spawn(async move {
                if let Err(e) = Self::handle_packet(
                    core_socket.deref(),
                    buffer,
                    blacklist.deref(),
                    origin_addr,
                    number_of_bytes,
                    proxy.as_str(),
                )
                .await
                {
                    log::error!("Error when handling packet: {e}");
                }
            });
        }
    }

    async fn handle_packet(
        socket: &UdpSocket,
        buffer: [u8; 4096],
        blacklist: &HashSet<String>,
        origin_addr: SocketAddr,
        number_of_bytes: usize,
        proxy: &str,
    ) -> Result<()> {
        let packet = dns_parser::Packet::parse(&buffer[..number_of_bytes])?;

        // TODO: Treat all questions !
        if blacklist.contains(packet.questions.first().unwrap().qname.to_string().as_str()) {
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
