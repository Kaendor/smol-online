use bevy::app::{App, Plugin};

pub struct ServerPlugin;

mod network;
mod teams;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((teams::TeamsPlugin, network::NetworkPlugin));
    }
}
