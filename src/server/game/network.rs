use self::systems::{handle_connections, handle_tile_selection, setup_server};
use bevy::app::{App, Plugin, Startup, Update};
use shared::{events::TileSelection, GameSharedPlugin};

mod systems;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GameSharedPlugin)
            .add_systems(Startup, setup_server)
            .add_systems(Update, (handle_connections, handle_tile_selection));
    }
}
