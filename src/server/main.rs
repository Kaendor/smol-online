use bevy::{app::App, DefaultPlugins};
use game::ServerPlugin;

mod game;

fn main() {
    App::new().add_plugins((DefaultPlugins, ServerPlugin)).run();
}
