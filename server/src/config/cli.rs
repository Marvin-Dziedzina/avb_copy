use std::net::IpAddr;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    version,
    about = "The AVB dedicated server CLI. Use the CLI to temporarily override config.toml."
)]
pub struct Cli {
    #[arg(long, default_value = None)]
    pub ip: Option<IpAddr>,
    #[arg(long, short, default_value = None)]
    pub port: Option<u16>,

    #[arg(long, short, default_value = None)]
    pub max_players: Option<u32>,
}

#[derive(Debug)]
pub struct ValidatedCli {
    pub ip: IpAddr,
    pub port: u16,

    pub max_players: u32,
}

impl Cli {
    /// Returns true if all fields are [`Some`].
    pub fn into_validated(self) -> Option<ValidatedCli> {
        match self {
            Self {
                ip: Some(ip),
                port: Some(port),
                max_players: Some(max_players),
            } => Some(ValidatedCli {
                ip,
                port,
                max_players,
            }),
            _ => None,
        }
    }
}
