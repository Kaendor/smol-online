use bevy::{
    app::{App, PluginGroup},
    diagnostic::DiagnosticsPlugin,
    hierarchy::HierarchyPlugin,
    log::LogPlugin,
    transform::TransformPlugin,
    utils::default,
    window::WindowPlugin,
    winit::WinitPlugin,
    DefaultPlugins, MinimalPlugins,
};
use game::ServerPlugin;

mod game;

fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins,
            ServerPlugin,
            LogPlugin::default(),
            TransformPlugin,
            HierarchyPlugin,
            DiagnosticsPlugin,
        ))
        .run();
}
