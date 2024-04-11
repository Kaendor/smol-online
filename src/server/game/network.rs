use std::{net::UdpSocket, time::SystemTime};

use bevy::{
    app::{App, Plugin, Startup},
    ecs::system::Commands,
    log::info,
};
use bevy_renet::{
    renet::{
        transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
        ConnectionConfig, RenetServer,
    },
    transport::NetcodeServerPlugin,
    RenetServerPlugin,
};

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((RenetServerPlugin, NetcodeServerPlugin))
            .add_systems(Startup, setup_server);
    }
}

fn setup_server(mut commands: Commands) {
    let server = RenetServer::new(ConnectionConfig::default());
    commands.insert_resource(server);

    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();
    let server_config = ServerConfig {
        current_time: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap(),
        max_clients: 64,
        protocol_id: 0,
        public_addresses: vec![server_addr],
        authentication: ServerAuthentication::Unsecure,
    };
    let transport = NetcodeServerTransport::new(server_config, socket).unwrap();

    info!("Server started at {}", server_addr);
    commands.insert_resource(transport);
}

#[cfg(test)]
mod tests {
    use std::{net::UdpSocket, time::SystemTime};

    use bevy::{
        app::App,
        ecs::{
            event::{EventReader, Events},
            system::Res,
        },
        time::TimePlugin,
        DefaultPlugins, MinimalPlugins,
    };
    use bevy_renet::{
        renet::{
            transport::{ClientAuthentication, NetcodeClientTransport},
            ConnectionConfig, RenetClient, ServerEvent,
        },
        transport::NetcodeClientPlugin,
        RenetClientPlugin,
    };

    use super::NetworkPlugin;

    fn network_app() -> App {
        let mut app = App::new();

        app.add_plugins((NetworkPlugin, MinimalPlugins));

        app
    }

    fn client_app() -> App {
        let mut app = App::new();

        app.add_plugins((RenetClientPlugin, MinimalPlugins));

        let client = RenetClient::new(ConnectionConfig::default());
        app.insert_resource(client);

        // Setup the transport layer
        app.add_plugins(NetcodeClientPlugin);

        let authentication = ClientAuthentication::Unsecure {
            server_addr: "127.0.0.1:5000".parse().unwrap(),
            client_id: 0,
            user_data: None,
            protocol_id: 0,
        };
        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();

        app.insert_resource(transport);

        app
    }

    #[test]
    fn client_can_connect() {
        let mut server = network_app();
        server.update();

        let mut client = client_app();
        client.update();

        server.update();
        client.update();
        server.update();

        let reader = server.world.resource_mut::<Events<ServerEvent>>();
        assert!(!reader.is_empty());
    }
}
