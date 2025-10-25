use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use bevy::prelude::*;
use lightyear::{
    link::Link,
    netcode::{ConnectToken, Key, NetcodeClient},
    prelude::{
        Authentication, Client, Connect, Disconnect, LocalAddr, PeerAddr, ReplicationReceiver,
        UdpIo, client::NetcodeConfig,
    },
};
use log::info;
use protocol::PROTOCOL_ID;

use crate::{config::Config, states::AppState};

#[derive(Debug)]
pub struct NetworkPlugins;

impl Plugin for NetworkPlugins {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);

        app.add_observer(join_game_observer)
            .add_observer(leave_game_observer);
    }
}

#[derive(Debug, Component)]
pub struct LocalClient;

#[derive(Event)]
pub enum JoinGameEvent {
    Token {
        address: SocketAddr,
        token: ConnectToken,
    },
    #[cfg(debug_assertions)]
    Manual { address: SocketAddr, key: Key },
}

#[derive(Debug, Event)]
pub struct LeaveGameEvent;

fn setup(mut commands: Commands) {
    commands.spawn((
        LocalClient,
        Client::default(),
        LocalAddr(SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0)),
        Link::new(None),
        ReplicationReceiver::default(),
        UdpIo::default(),
    ));
}

fn join_game_observer(
    trigger: On<JoinGameEvent>,
    mut commands: Commands,
    client: Query<Entity, (With<LocalClient>, With<Client>)>,
    mut next_state: ResMut<NextState<AppState>>,
    mut config: ResMut<Config>,
) {
    let (address, auth) = match trigger.event() {
        JoinGameEvent::Token { address, token } => (address, Authentication::Token(token.clone())),
        #[cfg(debug_assertions)]
        JoinGameEvent::Manual { address, key } => (
            address,
            Authentication::Manual {
                server_addr: *address,
                client_id: config.client_id,
                private_key: *key,
                protocol_id: PROTOCOL_ID,
            },
        ),
    };

    info!("Joining {}", address);

    config.peer_address = Some(*address);

    let client_entity = client.single().expect("Only one `LocalClient` can exist");
    // Add components that are needed to establish connection.
    commands.entity(client_entity).insert((
        PeerAddr(*address),
        NetcodeClient::new(auth, NetcodeConfig::default()).unwrap(),
    ));
    commands.trigger(Connect {
        entity: client_entity,
    });

    next_state.set(AppState::InGame);

    info!("Connected to {}", address);
}

fn leave_game_observer(
    _: On<LeaveGameEvent>,
    mut commands: Commands,
    client: Query<Entity, (With<LocalClient>, With<Client>)>,
    mut next_state: ResMut<NextState<AppState>>,
    mut config: ResMut<Config>,
) {
    config.peer_address = None;

    let client_entity = client.single().expect("Only one `LocalClient` can exist");
    commands.trigger(Disconnect {
        entity: client_entity,
    });
    commands
        .entity(client_entity)
        .remove::<(PeerAddr, NetcodeClient)>();

    next_state.set(AppState::MainMenu);

    info!("Disconnected from server");
}
