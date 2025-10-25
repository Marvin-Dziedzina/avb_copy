use std::time::Duration;

use bevy::{log::LogPlugin, prelude::*, state::app::StatesPlugin};
use common::CommonPlugins;
use lightyear::prelude::server::*;

use crate::{
    config::ConfigPlugins, editor::EditorPlugins, game::GamePlugins, network::NetworkPlugins,
};

mod config;
mod editor;
mod game;
mod network;

fn main() {
    let mut app = App::new();

    // Bevy plugins and config/
    app.add_plugins((MinimalPlugins, LogPlugin::default(), StatesPlugin));

    // Lightyear plugins and config.
    app.add_plugins(ServerPlugins {
        tick_duration: protocol::TICK_DURATION,
    });
    app.add_plugins(protocol::ProtocolPlugins);

    // Custom plugins and config.
    app.add_plugins((
        ConfigPlugins,
        CommonPlugins,
        NetworkPlugins,
        GamePlugins,
        EditorPlugins,
    ));

    app.run();
}
