use bevy::prelude::*;

pub struct GraphPlugin;

#[derive(Default, Debug, Copy, Clone, Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Node;

#[derive(Component, Copy, Clone)]
struct NodeId(usize);

#[derive(Debug, Component)]
struct Edge;

#[derive(Component)]
struct SrcDst(NodeId, NodeId);

struct NodeData {
    id: NodeId,
    pos: Position,
}

struct EdgeData {
    src: NodeId,
    dst: NodeId,
}

impl Plugin for GraphPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (update_node, update_edge));
    }
}

fn generate_random_data(node_num: usize, edge_num: usize) -> (Vec<NodeData>, Vec<EdgeData>) {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    for i in 0..node_num {
        nodes.push(NodeData {
            id: NodeId(i),
            // -300 to 300
            pos: Position {
                x: rand::random::<f32>() * 600. - 300.,
                y: rand::random::<f32>() * 600. - 300.,
            },
        });
    }

    let mut edge_count = 0;
    while edge_count < edge_num {
        let src = rand::random::<usize>() % node_num;
        let dst = rand::random::<usize>() % node_num;
        if src != dst {
            edges.push(EdgeData {
                src: NodeId(src),
                dst: NodeId(dst),
            });
            edge_count += 1;
        }
    }

    (nodes, edges)
}

fn setup(mut commands: Commands) {
    // generate random data
    let (nodes, edges) = generate_random_data(10, 10);

    commands.spawn(Camera2dBundle::default());

    for node in nodes.iter() {
        commands.spawn((Node, node.id, node.pos));
    }

    for edge in edges.iter() {
        commands.spawn((Edge, SrcDst(edge.src, edge.dst)));
    }
}

fn update_node(mut gizmos: Gizmos, node_pos_query: Query<&Position, With<Node>>) {
    for node_pos in node_pos_query.iter() {
        let color = Color::WHITE;
        gizmos.circle_2d(Vec2::new(node_pos.x, node_pos.y), 10., color);
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
        }
        gizmos.line_2d(src_vec, dst_vec, Color::WHITE);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_data() {
        let (nodes, edge) = generate_random_data(10, 3);
        // print nodes (test with --nocapture)
        for node in nodes.iter() {
            println!("node id: {:?}, pos: {:?}", node.id.0, node.pos);
        }
        // print nodes (test with --nocapture)
        for edge in edge.iter() {
            println!("edge src: {:?}, dst: {:?}", edge.src.0, edge.dst.0);
        }
    }
}
