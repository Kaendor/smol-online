use bevy::app::{App, Plugin, Startup, Update};
use bevy::prelude::*;
use lightyear::prelude::server::*;
use lightyear::prelude::*;

use self::systems::{create_player_on_connection, setup_server};
use shared::{GameSharedPlugin, SERVER_ADDR};

mod systems;

pub struct NetworkPlugin;

/// Here we create the lightyear [`ServerPlugins`]
fn build_server_plugin() -> ServerPlugins {
    // The IoConfig will specify the transport to use.
    let io = IoConfig {
        // the address specified here is the server_address, because we open a UDP socket on the server
        transport: ServerTransport::UdpSocket(SERVER_ADDR),
        ..default()
    };

    // The NetConfig specifies how we establish a connection with the server.
    // We can use either Steam (in which case we will use steam sockets and there is no need to specify
    // our own io) or Netcode (in which case we need to specify our own io).
    let net_config = NetConfig::Netcode {
        io,
        config: NetcodeConfig::default(),
    };

    let config = ServerConfig {
        // part of the config needs to be shared between the client and server
        shared: SharedConfig::default(),
        // we can specify multiple net configs here, and the server will listen on all of them
        // at the same time. Here we will only use one
        net: vec![net_config],
        ..default()
    };

    ServerPlugins::new(config)
}

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GameSharedPlugin)
            .add_plugins(build_server_plugin())
            .add_systems(Startup, setup_server)
            .add_systems(Update, create_player_on_connection);
    }
}
