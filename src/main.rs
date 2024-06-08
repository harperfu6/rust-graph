use bevy::prelude::*;
use rust_graph::graph::GraphPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GraphPlugin)
        .run();
}
