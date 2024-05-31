use std::{
    net::{Ipv4Addr, SocketAddr, UdpSocket},
    time::SystemTime,
};

use bevy::{
    ecs::{event::EventReader, system::Res},
    log::info,
};
use bevy_replicon::prelude::*;
use bevy_replicon_renet::{
    renet::{
        transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
        ConnectionConfig, RenetServer,
    },
    RenetChannelsExt,
};

use bevy::ecs::system::Commands;

use super::TileSelection;

const PORT: u16 = 5000;
const PROTOCOL_ID: u64 = 0;

pub fn handle_tile_selection(
    mut tile_selection_events: EventReader<FromClient<TileSelection>>,
    mut server: Res<RenetServer>,
) {
    for FromClient { client_id, event } in tile_selection_events.read() {
        let position = event.0;
        info!(client = client_id.get(), "Tile selected: {position:?}");
    }
}

pub fn handle_connections(mut server_events: EventReader<ServerEvent>) {
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                info!("{client_id:?} connected");
                // Generate pseudo random color from client id.
                // let r = ((client_id.get() % 23) as f32) / 23.0;
                // let g = ((client_id.get() % 27) as f32) / 27.0;
                // let b = ((client_id.get() % 39) as f32) / 39.0;
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                info!("{client_id:?} disconnected: {reason}");
            }
        }
    }
}

pub(crate) fn setup_server(mut commands: Commands, channels: Res<RepliconChannels>) {
    let server_channels_config = channels.get_server_configs();
    let client_channels_config = channels.get_client_configs();

    let server = RenetServer::new(ConnectionConfig {
        server_channels_config,
        client_channels_config,
        ..Default::default()
    });

    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Failed to get current time");
    let public_addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), PORT);
    let socket = UdpSocket::bind(public_addr).expect("Failed to bind socket");
    let server_config = ServerConfig {
        current_time,
        max_clients: 10,
        protocol_id: PROTOCOL_ID,
        authentication: ServerAuthentication::Unsecure,
        public_addresses: vec![public_addr],
    };
    let transport =
        NetcodeServerTransport::new(server_config, socket).expect("Failed to create transport");

    commands.insert_resource(server);
    commands.insert_resource(transport);
}
