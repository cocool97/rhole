mod models;

pub use crate::models::Config;

use anyhow::Result;
use clap::Parser;
use dns_parser::Packet;
use models::Opts;
use std::collections::{HashSet};
use tokio::net::UdpSocket;

async fn listen(socket: &UdpSocket, nogoes: &HashSet<&str>) -> Result<usize> {
    let mut buffer = [0u8; 4096];

    let (number_of_bytes, src_addr) = socket
        .recv_from(&mut buffer)
        .await?;

    let packet = Packet::parse(&buffer[..number_of_bytes])?;

    if nogoes.contains(packet.questions.first().unwrap().qname.to_string().as_str()) {
        println!("NO GO");
    } else {
        socket.connect("192.168.1.1:53").await?;

        socket.send(&buffer[..number_of_bytes]).await?;

        let s2 = socket.recv(&mut buffer).await?;

        socket.connect(src_addr).await?;
        socket.send(&buffer[..s2]).await?;
    }

    Ok(number_of_bytes)
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::parse();
    let config = Config::from_file(opts.config_path).await?;

    let socket = UdpSocket::bind((config.net.listen_addr, config.net.listen_port)).await?;

    let mut nogoes = HashSet::new();
    nogoes.insert("google.com");
    nogoes.insert("toto.com");

    listen(&socket, &nogoes).await?;

    Ok(())

    // Improvment :
    // - RUST WEB frontend ?
}
