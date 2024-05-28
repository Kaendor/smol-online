//! This module contains the shared code between the client and the server.
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use bevy::prelude::*;
use bevy_replicon::RepliconPlugins;

pub const FIXED_TIMESTEP_HZ: f64 = 64.0;

pub const SERVER_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 5000);

/// The shared plugin is used to register shared resources and systems
#[derive(Clone)]
pub struct GameSharedPlugin;

impl Plugin for GameSharedPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RepliconPlugins);
    }
}
