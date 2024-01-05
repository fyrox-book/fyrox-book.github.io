use fyrox::{
    core::{
        algebra::Vector3,
        pool::Handle,
        reflect::prelude::*,
        uuid::{uuid, Uuid},
        variable::InheritableVariable,
        visitor::prelude::*,
        TypeUuidProvider,
    },
    impl_component_provider,
    scene::{graph::physics::RayCastOptions, node::Node},
    script::{ScriptContext, ScriptTrait},
};

#[derive(Visit, Reflect, Default, Debug, Clone)]
pub struct Projectile {
    // ANCHOR: trail_field
    #[visit(optional)]
    trail: InheritableVariable<Handle<Node>>,
    // ANCHOR_END: trail_field
}

impl_component_provider!(Projectile);

impl TypeUuidProvider for Projectile {
    fn type_uuid() -> Uuid {
        uuid!("55199744-33be-4c1a-832a-727fe5f0ea28")
    }
}

impl ScriptTrait for Projectile {
    // ANCHOR: on_start
    fn on_start(&mut self, ctx: &mut ScriptContext) {
        let this_node = &ctx.scene.graph[ctx.handle];
        let this_node_position = this_node.global_position();

        // Cast a ray in from the node in its "look" direction.
        let mut intersections = Vec::new();
        ctx.scene.graph.physics.cast_ray(
            RayCastOptions {
                ray_origin: this_node_position.into(),
                ray_direction: this_node.look_vector(),
                max_len: 1000.0,
                groups: Default::default(),
                // Sort results of the ray casting so the closest intersection will be in the
                // beginning of the list.
                sort_results: true,
            },
            &mut intersections,
        );

        let trail_length = if let Some(intersection) = intersections.first() {
            // If we got an intersection, scale the trail by the distance between the position of the node
            // with this script and the intersection position.
            this_node_position.metric_distance(&intersection.position.coords)
        } else {
            // Otherwise the trail will be as large as possible.
            1000.0
        };

        if let Some(trail_node) = ctx.scene.graph.try_get_mut(*self.trail) {
            let transform = trail_node.local_transform_mut();
            let current_trail_scale = **transform.scale();
            transform.set_scale(Vector3::new(
                // Keep x scaling.
                current_trail_scale.x,
                trail_length,
                // Keep z scaling.
                current_trail_scale.z,
            ));
        }
    }
    // ANCHOR_END: on_start

    fn id(&self) -> Uuid {
        Self::type_uuid()
    }
}
