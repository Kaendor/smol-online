use bevy::app::{App, Plugin};
use bevy::prelude::*;

use self::inputs::PlayerInputPlugin;
use self::network::NetworkPlugin;

mod inputs;
mod network;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_plugins((NetworkPlugin, PlayerInputPlugin));
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[inline]
pub fn world_to_grid_pos(world: Vec2) -> IVec2 {
    world.floor().as_ivec2()
}
