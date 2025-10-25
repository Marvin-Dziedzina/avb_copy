use std::{
    fs,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
};

use bevy::prelude::*;
use clap::Parser;
use common::ip_addr_into_socket_addr;
use serde::{Deserialize, Serialize};

use crate::config::cli::Cli;

mod cli;

/// FIXME: Use directories crate.
const CONFIG_PATH: &str = "./config.toml";

pub struct ConfigPlugins;

impl Plugin for ConfigPlugins {
    fn build(&self, app: &mut App) {
        app.insert_resource(Config::new().expect("Failed to get config"));
    }
}

#[derive(Debug, Resource, Serialize, Deserialize)]
pub struct Config {
    pub addr: SocketAddr,
    pub max_players: u32,
}

impl Config {
    pub fn new() -> Result<Self, BevyError> {
        let cli = Cli::parse();

        if let Some(validated_cli) = cli.into_validated() {
            Ok(Self {
                addr: ip_addr_into_socket_addr(validated_cli.ip, validated_cli.port),
                max_players: validated_cli.max_players,
            })
        } else {
            Self::open()
        }
    }

    pub fn open() -> Result<Self, BevyError> {
        match fs::read_to_string(CONFIG_PATH) {
            Ok(contents) => match toml::from_str(&contents) {
                Ok(config) => Ok(config),
                Err(e) => {
                    log::warn!("Failed to serialize server config falling back to default: {e}");
                    let config = Self::default();
                    Self::write_to_file(&config)?;

                    Ok(config)
                }
            },
            Err(e) => {
                warn!("Failed to read file: {}", e);
                let config = Self::default();
                Self::write_to_file(&config)?;

                Ok(config)
            }
        }
    }

    pub fn write(&self) -> Result<(), BevyError> {
        Self::write_to_file(self)
    }

    fn write_to_file(config: &Self) -> Result<(), BevyError> {
        fs::write(CONFIG_PATH, toml::to_string_pretty(config)?).map_err(|e| e.into())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            addr: SocketAddr::V4(SocketAddrV4::new(
                Ipv4Addr::new(127, 0, 0, 1),
                common::DEFAULT_PORT,
            )),
            max_players: 4,
        }
    }
}
