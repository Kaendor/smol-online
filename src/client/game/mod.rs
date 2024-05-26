use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use bevy::app::{App, Plugin};
use bevy::prelude::*;
use lightyear::shared::config::SharedConfig;
use lightyear::{client::plugin::ClientPlugins, connection::netcode::Key, prelude::client::*};
use shared::{GameSharedPlugin, SERVER_ADDR};

pub struct ClientPlugin;

const CLIENT_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 4000);

/// Here we create the lightyear [`ClientPlugins`]
fn build_client_plugin() -> ClientPlugins {
    // Authentication is where you specify how the client should connect to the server
    // This is where you provide the server address.
    let auth = Authentication::Manual {
        server_addr: SERVER_ADDR,
        client_id: 0,
        private_key: Key::default(),
        protocol_id: 0,
    };
    // The IoConfig will specify the transport to use.
    let io = IoConfig {
        // the address specified here is the client_address, because we open a UDP socket on the client
        transport: ClientTransport::UdpSocket(CLIENT_ADDR),
        ..default()
    };
    // The NetConfig specifies how we establish a connection with the server.
    // We can use either Steam (in which case we will use steam sockets and there is no need to specify
    // our own io) or Netcode (in which case we need to specify our own io).
    let net_config = NetConfig::Netcode {
        auth,
        io,
        config: NetcodeConfig::default(),
    };
    let config = ClientConfig {
        // part of the config needs to be shared between the client and server
        shared: SharedConfig::default(),
        net: net_config,
        ..default()
    };
    ClientPlugins::new(config)
}

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(build_client_plugin())
            .add_systems(Startup, connect_client)
            .add_plugins(GameSharedPlugin);
    }
}

fn connect_client(mut commands: Commands) {
    commands.connect_client();
}
