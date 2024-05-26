use bevy::{
    app::App, diagnostic::DiagnosticsPlugin, hierarchy::HierarchyPlugin, log::LogPlugin,
    transform::TransformPlugin, MinimalPlugins,
};
use game::ServerPlugin;

use shared::GameSharedPlugin;

mod game;

fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins,
            GameSharedPlugin,
            ServerPlugin,
            LogPlugin::default(),
            TransformPlugin,
            HierarchyPlugin,
            DiagnosticsPlugin,
        ))
        .run();
}
