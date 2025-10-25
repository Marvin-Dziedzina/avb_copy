use bevy::prelude::*;

#[derive(Debug)]
pub struct StatesPlugins;

impl Plugin for StatesPlugins {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_sub_state::<GameState>()
            .add_observer(despawn_entities_observer);
    }
}

/// The app state.
#[derive(Debug, States, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
}

/// The game state. Only while [`AppState::InGame`].
#[derive(Debug, SubStates, Clone, PartialEq, Eq, Hash, Default)]
#[source(AppState = AppState::InGame)]
pub enum GameState {
    #[default]
    InGame,
    InEditor,
    Pause,
}

/// Despawns a list of entities.
#[derive(Debug, Event)]
pub struct DespawnEntities(Vec<Entity>);

/// Despawn all entities in [`DespawnEntities`].
pub fn despawn_entities_observer(
    mut despawn_entities: On<DespawnEntities>,
    mut commands: Commands,
) {
    let entities: Vec<Entity> = std::mem::take(&mut despawn_entities.0);
    for entity in entities {
        commands.entity(entity).despawn();
    }
}
