use bevy::app::{App, Plugin, Startup, Update};
use bevy_replicon::prelude::*;
use bevy_replicon_renet::RepliconRenetPlugins;

use self::systems::{handle_connections, setup_server};

mod systems;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RepliconPlugins)
            .add_plugins(RepliconRenetPlugins)
            .add_systems(Startup, setup_server)
            .add_systems(Update, handle_connections);
    }
}
