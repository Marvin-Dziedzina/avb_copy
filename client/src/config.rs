use std::net::SocketAddr;

use bevy::prelude::*;
use clap::Parser;

use cli::Cli;
use client::get_client_id;
use common::{DEFAULT_PORT, ip_addr_into_socket_addr};
use lightyear::netcode;

mod cli;

#[derive(Debug)]
pub struct ConfigPlugins;

impl Plugin for ConfigPlugins {
    fn build(&self, app: &mut App) {
        app.insert_resource(Config::new());

        app.add_systems(Startup, setup);
    }
}

#[derive(Debug, Resource)]
pub struct Config {
    pub peer_address: Option<SocketAddr>,
    #[cfg(debug_assertions)]
    pub client_id: u64,
}

impl Config {
    fn new() -> Self {
        let cli_args = Cli::parse();

        let address = if cli_args.address.is_some() {
            cli_args.address
        } else {
            cli_args
                .ip
                .map(|ip| ip_addr_into_socket_addr(ip, cli_args.port.unwrap_or(DEFAULT_PORT)))
        };
        #[cfg(debug_assertions)]
        let client_id = cli_args.client_id.unwrap_or(get_client_id());
        #[cfg(debug_assertions)]
        info!("Client ID: {}", client_id);

        Self {
            peer_address: address,
            #[cfg(debug_assertions)]
            client_id,
        }
    }
}

fn setup(mut commands: Commands, config: Res<Config>) {
    if let Some(addr) = config.peer_address {
        use crate::network::JoinGameEvent;

        let key = netcode::Key::default();

        info!("Remote Address: {}", addr);
        info!("Key: {}", {
            let mut key_string = key
                .iter()
                .map(|byte| format!(" {},", byte))
                .collect::<String>();

            key_string = key_string.trim().to_string();
            key_string.remove(key_string.len() - 1);
            key_string.insert(0, '[');
            key_string.push(']');

            key_string
        });

        // TODO: Should not use Manual in release. Change after Token exchange is done.
        commands.trigger(JoinGameEvent::Manual { address: addr, key });
    };
}
