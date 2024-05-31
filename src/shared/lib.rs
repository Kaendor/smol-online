//! This module contains the shared code between the client and the server.
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use bevy::prelude::*;
use bevy_replicon::{
    client::events::ClientEventAppExt, core::channels::ChannelKind, RepliconPlugins,
};
use bevy_replicon_renet::RepliconRenetPlugins;
use events::TileSelection;

pub mod events;

pub const FIXED_TIMESTEP_HZ: f64 = 64.0;
pub const PROTOCOL_ID: u64 = 0;
pub const PORT: u16 = 5000;

pub const SERVER_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 5000);
/// The shared plugin is used to register shared resources and systems
#[derive(Clone)]
pub struct GameSharedPlugin;

impl Plugin for GameSharedPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RepliconPlugins)
            .add_plugins(RepliconRenetPlugins)
            .add_client_event::<TileSelection>(ChannelKind::Ordered);
    }
}
