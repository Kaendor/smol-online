use std::net::{Ipv4Addr, UdpSocket};
use std::time::SystemTime;

use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy_replicon::core::channels::RepliconChannels;
use bevy_replicon_renet::renet::transport::{ClientAuthentication, NetcodeClientTransport};
use bevy_replicon_renet::renet::{ConnectionConfig, RenetClient};
use bevy_replicon_renet::RenetChannelsExt;
use shared::{GameSharedPlugin, PROTOCOL_ID, SERVER_ADDR};

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, connect_client)
            .add_plugins(GameSharedPlugin);
    }
}

fn connect_client(mut commands: Commands, channels: Res<RepliconChannels>) {
    let server_channels_config = channels.get_server_configs();
    let client_channels_config = channels.get_client_configs();

    let client = RenetClient::new(ConnectionConfig {
        server_channels_config,
        client_channels_config,
        ..Default::default()
    });

    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Failed to get current time");
    let client_id = current_time.as_millis() as u64;
    let socket = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).expect("Failed to bind socket");

    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr: SERVER_ADDR,
        user_data: None,
    };
    let transport = NetcodeClientTransport::new(current_time, authentication, socket)
        .expect("Failed to create transport");

    commands.insert_resource(client);
    commands.insert_resource(transport);

    commands.spawn(TextBundle::from_section(
        format!("Client: {client_id:?}"),
        TextStyle {
            font_size: 30.0,
            color: Color::WHITE,
            ..default()
        },
    ));
}
