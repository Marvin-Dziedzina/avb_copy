use std::net::IpAddr;
#[cfg(debug_assertions)]
use std::net::SocketAddr;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about = "The CLI interface of the AVB client.")]
pub struct Cli {
    #[arg(short, long, default_value = None)]
    pub address: Option<SocketAddr>,
    #[arg(long, default_value = None)]
    pub ip: Option<IpAddr>,
    #[arg(short, long, default_value = None)]
    pub port: Option<u16>,

    #[cfg(debug_assertions)]
    #[arg(short, long, default_value = None)]
    pub client_id: Option<u64>,
}
