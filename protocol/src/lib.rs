use bevy::prelude::*;

use crate::{
    channels::ProtocolChannelsPlugin, components::ProtocolComponentsPlugin,
    inputs::ProtocolInputsPlugin, messages::ProtocolMessagesPlugin,
};

mod channels;
mod components;
mod inputs;
mod messages;

pub const PROTOCOL_ID: u64 = 0;

pub const TARGET_TICK_RATE: u64 = 60;
pub const TICK_DURATION: std::time::Duration =
    std::time::Duration::from_millis(1000 / TARGET_TICK_RATE);

pub struct ProtocolPlugins;

impl Plugin for ProtocolPlugins {
    fn build(&self, app: &mut App) {
        // Protocol tutorial
        // https://cbournhonesque.github.io/lightyear/book/tutorial/setup.html
        app.add_plugins((
            ProtocolInputsPlugin,
            ProtocolMessagesPlugin,
            ProtocolComponentsPlugin,
            ProtocolChannelsPlugin,
        ));
    }
}
