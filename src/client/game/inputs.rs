use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::*,
    window::PrimaryWindow,
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
) {
    for event in mouse_button_input.read() {
        if event.state == ButtonState::Pressed {
            let window = windows.single();

            let player_position = window.cursor_position().unwrap_or_default();
            println!("Clicked at {:?}", player_position);

            click_events.send(TileSelection(player_position));
        }
    }
}
