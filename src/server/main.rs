use bevy::{
    app::App, diagnostic::DiagnosticsPlugin, hierarchy::HierarchyPlugin, log::LogPlugin,
    transform::TransformPlugin, MinimalPlugins,
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
