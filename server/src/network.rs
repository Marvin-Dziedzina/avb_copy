use bevy::prelude::*;
use lightyear::prelude::{LocalAddr, server::*};
use log::info;

use crate::config::Config;

pub struct NetworkPlugins;

impl Plugin for NetworkPlugins {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, config: Res<Config>) {
    info!("Starting server...");

    let server_entity = commands
        .spawn((
            NetcodeServer::new(NetcodeConfig::default()),
            LocalAddr(config.addr),
            ServerUdpIo::default(),
        ))
        .id();

    commands.trigger(Start {
        entity: server_entity,
    });

    info!("Server started on {}", config.addr);
    info!("Max players: {}", config.max_players);
}
