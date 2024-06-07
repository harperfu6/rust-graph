use bevy::prelude::*;
#[derive(Default, Component)]
struct Position {
    x: f32,
    y: f32,
}
#[derive(Component)]

struct Node;

#[derive(Component)]
struct NodeId(usize);

#[derive(Component)]
struct Edge;

#[derive(Component)]
struct SrcDst(NodeId, NodeId);

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((Node, NodeId(1), Position { x: 0.0, y: 0.0 }));
    commands.spawn((Node, NodeId(2), Position { x: 100.0, y: 0.0 }));

    commands.spawn((Edge, SrcDst(NodeId(1), NodeId(2))));
}

fn update_node(mut gizmos: Gizmos, node_pos_query: Query<&Position, With<Node>>) {
    for node_pos in node_pos_query.iter() {
        let color = Color::WHITE;
        gizmos.circle_2d(Vec2::new(node_pos.x, node_pos.y), 20., color);
    }
}

fn update_edge(
    mut gizmos: Gizmos,
    src_dst_query: Query<&SrcDst>,
    node_query: Query<(&Position, &NodeId)>,
) {
    for src_dst in src_dst_query.iter() {
        let mut src_vec = Vec2::new(0., 0.);
        let mut dst_vec = Vec2::new(0., 0.);
        for (node_pos, node_id) in node_query.iter() {
            if src_dst.0 .0 == node_id.0 {
                src_vec = Vec2::new(node_pos.x, node_pos.y);
            } else if src_dst.1 .0 == node_id.0 {
                dst_vec = Vec2::new(node_pos.x, node_pos.y);
            }
            gizmos.line_2d(src_vec, dst_vec, Color::WHITE);
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (update_node, update_edge))
        .run();
}
