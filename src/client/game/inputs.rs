use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::*,
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
) {
    for event in mouse_button_input.read() {
        if event.state == ButtonState::Released {
            println!("Mouse button released!");
        }

        if event.state == ButtonState::Pressed {
            println!("Mouse button pressed!");
            let player_position = Vec2::ZERO;

            click_events.send(TileSelection(player_position));
        }
    }
}
