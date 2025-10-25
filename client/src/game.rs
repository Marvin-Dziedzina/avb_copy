use bevy::prelude::*;

use crate::game::world::WorldPlugin;

mod world;

pub struct GamePlugins;

impl Plugin for GamePlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(WorldPlugin);
    }
}
