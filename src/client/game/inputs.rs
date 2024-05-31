use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::*,
    window::PrimaryWindow,
};
use bevy_ecs_tilemap::{
    map::{TilemapGridSize, TilemapSize, TilemapType},
    tiles::{TilePos, TileStorage},
};
use shared::events::TileSelection;

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, handle_click);
    }
}

fn handle_click(
    mut mouse_button_input: EventReader<MouseButtonInput>,
    mut click_events: EventWriter<TileSelection>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&GlobalTransform, &Camera)>,
    tilemap: Query<(&TilemapSize, &TilemapGridSize, &TilemapType, &TileStorage)>,
) {
    for event in mouse_button_input.read() {
        if event.state == ButtonState::Pressed {
            let window = windows.single();
            let (cam_t, camera) = camera.single();
            let player_position = window.cursor_position().unwrap_or_default();

            let Some(position) = camera.viewport_to_world_2d(cam_t, player_position) else {
                info!("Cursor outside of window");
                return;
            };

            let (map_size, grid_size, map_type, storage) = tilemap.single();

            let Some(tile) = TilePos::from_world_pos(&position, map_size, grid_size, map_type)
            else {
                return;
            };

            info!("Clicked on tile {:?}", tile);

            click_events.send(TileSelection(UVec2::new(tile.x, tile.y)));
        }
    }
}
