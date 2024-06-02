use fyrox::core::algebra::Vector3;
use fyrox::core::math::TriangleDefinition;
use fyrox::graph::SceneGraph;
use fyrox::scene::navmesh::NavigationalMesh;
use fyrox::scene::Scene;
use fyrox::utils::navmesh::{Navmesh, NavmeshAgent};

// ANCHOR: make_navmesh
fn make_navmesh(scene: &Scene, navmesh_name: &str) -> Navmesh {
    // Find mesh node in existing scene and create navigation mesh from it.
    let navmesh_node_handle = scene.graph.find_by_name_from_root(navmesh_name).unwrap().0;
    Navmesh::from_mesh(scene.graph[navmesh_node_handle].as_mesh())
}
// ANCHOR_END: make_navmesh

// ANCHOR: make_navmesh_from_vertices
fn make_navmesh_from_vertices() -> Navmesh {
    Navmesh::new(
        vec![
            TriangleDefinition([0, 1, 3]),
            TriangleDefinition([1, 2, 3]),
            TriangleDefinition([2, 5, 3]),
            TriangleDefinition([2, 4, 5]),
            TriangleDefinition([4, 7, 5]),
            TriangleDefinition([4, 6, 7]),
        ],
        vec![
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, 1.0),
            Vector3::new(1.0, 0.0, 1.0),
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(2.0, 0.0, 1.0),
            Vector3::new(2.0, 0.0, 0.0),
            Vector3::new(3.0, 0.0, 1.0),
            Vector3::new(3.0, 0.0, 0.0),
        ],
    )
}
// ANCHOR_END: make_navmesh_from_vertices

// ANCHOR: update_agent
fn update_agent(
    agent: &mut NavmeshAgent,
    target: Vector3<f32>,
    dt: f32,
    navmesh: &mut NavigationalMesh,
) {
    // Set the target to follow and the speed.
    agent.set_target(target);
    agent.set_speed(1.0);

    // Update the agent.
    let navmesh = navmesh.navmesh();
    agent.update(dt, &navmesh.read()).unwrap();

    // Print its position - you can use this position as target point of your game character.
    println!("{}", agent.position());
}
// ANCHOR_END: update_agent

// ANCHOR: find_navmesh
fn find_navmesh<'a>(scene: &'a mut Scene, name: &str) -> &'a mut NavigationalMesh {
    let handle = scene.graph.find_by_name_from_root(name).unwrap().0;
    scene.graph[handle].as_navigational_mesh_mut()
}
// ANCHOR_END: update_agent
