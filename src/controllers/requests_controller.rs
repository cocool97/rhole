use std::{collections::HashSet, net::SocketAddr, ops::Deref, sync::Arc};

use anyhow::{anyhow, Result};
use bytes::Bytes;
use dns_message_parser::rr::{A, RR};
use tokio::net::UdpSocket;

use crate::models::ProxyServer;

pub struct InboundConnectionsController {
    proxy_server: Arc<ProxyServer>,
    blacklist: Arc<HashSet<String>>,
}

impl InboundConnectionsController {
    pub fn new(proxy_server: ProxyServer, blacklist: HashSet<String>) -> Self {
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
            log::debug!("Received request from {}", origin_addr);

            let blacklist = self.blacklist.clone();
            let proxy = self.proxy_server.clone();
            let core_socket = socket.clone();
            let bytes = bytes::Bytes::copy_from_slice(&buffer[..number_of_bytes]);
            tokio::spawn(async move {
                if let Err(e) = Self::handle_packet(
                    core_socket.deref(),
                    bytes,
                    blacklist.deref(),
                    origin_addr,
                    proxy.deref(),
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
        buffer: Bytes,
        blacklist: &HashSet<String>,
        origin_addr: SocketAddr,
        proxy: &ProxyServer,
    ) -> Result<()> {
        let mut packet = dns_message_parser::Dns::decode(buffer.clone())?;
        let question = packet
            .questions
            .first()
            .ok_or_else(|| anyhow!("no questions in request..."))?;

        // TODO: Treat all questions !
        // TODO: Check trailing dots
        if blacklist.contains(question.domain_name.to_string().trim_end_matches('.')) {
            log::warn!(
                "Domain {} is blacklisted. Ignoring it.",
                question.domain_name
            );

            packet.answers = vec![RR::A(A {
                domain_name: question.domain_name.clone(),
                ttl: 86400,
                ipv4_addr: "0.0.0.0".parse()?,
            })];

            socket.send_to(&packet.encode()?, origin_addr).await?;
        } else {
            let proxy_socket = UdpSocket::bind("0.0.0.0:0").await?;
            proxy_socket.send_to(&buffer, proxy.to_addr()).await?;
            let mut res_buffer = [0u8; 4096];
            let received_size = proxy_socket.recv(&mut res_buffer).await?;
            socket
                .send_to(&res_buffer[..received_size], origin_addr)
                .await?;
        }

        Ok(())
    }
}
