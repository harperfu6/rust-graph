use bevy::prelude::*;

#[derive(Default, Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Default, Bundle)]
struct Node {
    position: Position,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn update(mut gizmos: Gizmos) {
    // nodes
    let nodes = [
        Node {
            position: Position { x: 0.0, y: 0.0 },
        },
        Node {
            position: Position { x: 100.0, y: 0.0 },
        },
    ];
    for node in nodes.into_iter() {
        let color = Color::WHITE;
        gizmos.circle_2d(Vec2::new(node.position.x, node.position.y), 20., color);
    }

    // edges
    gizmos.line_2d(Vec2::new(0., 0.), Vec2::new(100., 0.), Color::WHITE);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}
