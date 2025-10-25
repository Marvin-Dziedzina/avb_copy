use bevy::prelude::*;
use common::CommonPlugins;
use lightyear::prelude::client::*;

use crate::{
    config::ConfigPlugins,
    editor::EditorPlugins,
    game::GamePlugins,
    network::{LocalClient, NetworkPlugins},
    states::StatesPlugins,
};

mod config;
mod editor;
mod game;
mod network;
mod states;

fn main() {
    let mut app = App::new();

    // Bevy plugins and config.
    app.add_plugins(DefaultPlugins);

    // Lightyear plugins and config.
    app.add_plugins(ClientPlugins {
        tick_duration: protocol::TICK_DURATION,
    });
    app.add_plugins(protocol::ProtocolPlugins);

    // Custom plugins and config.
    app.add_plugins((
        NetworkPlugins,
        ConfigPlugins,
        CommonPlugins,
        StatesPlugins,
        GamePlugins,
        EditorPlugins,
    ));

    app.run();
}
