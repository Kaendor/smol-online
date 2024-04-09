use bevy::{app::App, DefaultPlugins};
use game::ClientPlugin;

mod game;

fn main() {
    App::new().add_plugins((DefaultPlugins, ClientPlugin)).run();
}
