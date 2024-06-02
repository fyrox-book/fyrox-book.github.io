use fyrox::{
    core::algebra::Vector3,
    utils::astar::{Graph, GraphVertex},
};

// ANCHOR: astar_on_uniform_grid
fn astar_on_uniform_grid() {
    // Create vertices.
    let size = 40;
    let mut vertices = Vec::new();
    for y in 0..size {
        for x in 0..size {
            vertices.push(GraphVertex::new(Vector3::new(x as f32, y as f32, 0.0)));
        }
    }
    let mut pathfinder = Graph::new();
    pathfinder.set_vertices(vertices);

    // Link vertices to form a uniform grid.
    for y in 0..(size - 1) {
        for x in 0..(size - 1) {
            pathfinder.link_bidirect(y * size + x, y * size + x + 1);
            pathfinder.link_bidirect(y * size + x, (y + 1) * size + x);
        }
    }

    // Build a path from vertex 0 to vertex 100.
    let mut path = Vec::new();
    assert!(pathfinder.build_positional_path(0, 100, &mut path).is_ok());
}
// ANCHOR_END: astar_on_uniform_grid
