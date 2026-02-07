use fyrox::core::pool::Handle;
use fyrox::scene::base::BaseBuilder;
use fyrox::scene::collider::{Collider, ColliderBuilder, ColliderShape};
use fyrox::scene::graph::Graph;

// ANCHOR: create_capsule_collider
fn create_capsule_collider(graph: &mut Graph) -> Handle<Collider> {
    ColliderBuilder::new(BaseBuilder::new())
        .with_shape(ColliderShape::capsule_y(0.5, 0.2))
        .with_friction(1.0)
        .build(graph)
}
// ANCHOR_END: create_capsule_collider
