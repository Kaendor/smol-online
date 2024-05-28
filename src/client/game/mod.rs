use std::net::{Ipv4Addr, SocketAddr, UdpSocket};
use std::time::SystemTime;

use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy_replicon::core::channels::RepliconChannels;
use bevy_replicon::RepliconPlugins;
use bevy_replicon_renet::renet::transport::{ClientAuthentication, NetcodeClientTransport};
use bevy_replicon_renet::renet::{ConnectionConfig, RenetClient};
use bevy_replicon_renet::{RenetChannelsExt, RepliconRenetPlugins};

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, connect_client)
            .add_systems(Startup, setup_camera)
            .add_plugins(RepliconPlugins)
            .add_plugins(RepliconRenetPlugins);
    }
}

const PORT: u16 = 5000;
const PROTOCOL_ID: u64 = 0;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn connect_client(mut commands: Commands, channels: Res<RepliconChannels>) {
    let server_channels_config = channels.get_server_configs();
    let client_channels_config = channels.get_client_configs();
    let ip = Ipv4Addr::LOCALHOST.into();

    let client = RenetClient::new(ConnectionConfig {
        server_channels_config,
        client_channels_config,
        ..Default::default()
    });

    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Failed to get current time");
    let client_id = current_time.as_millis() as u64;
    let server_addr = SocketAddr::new(ip, PORT);
    let socket = UdpSocket::bind((ip, 0)).expect("Failed to bind socket");
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
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
