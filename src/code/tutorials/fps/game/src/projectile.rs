use fyrox::graph::SceneGraph;
use fyrox::plugin::error::GameResult;
use fyrox::{
    core::{
        algebra::Vector3, math, pool::Handle, reflect::prelude::*, type_traits::prelude::*,
        variable::InheritableVariable, visitor::prelude::*,
    },
    resource::model::{ModelResource, ModelResourceExtension},
    scene::{graph::physics::RayCastOptions, node::Node, rigidbody::RigidBody},
    script::{ScriptContext, ScriptTrait},
};

#[derive(Visit, Reflect, Default, Debug, Clone, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "55199744-33be-4c1a-832a-727fe5f0ea28")]
#[visit(optional)]
pub struct Projectile {
    // ANCHOR: trail_field
    trail: InheritableVariable<Handle<Node>>,
    // ANCHOR_END: trail_field

    // ANCHOR: effect_field
    impact_effect: InheritableVariable<Option<ModelResource>>,
    // ANCHOR_END: effect_field
}

impl ScriptTrait for Projectile {
    // ANCHOR: on_start_begin
    fn on_start(&mut self, ctx: &mut ScriptContext) -> GameResult {
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

        let trail_node = ctx.scene.graph.try_get_mut(*self.trail)?;
        let transform = trail_node.local_transform_mut();
        let current_trail_scale = **transform.scale();
        transform.set_scale(Vector3::new(
            // Keep x scaling.
            current_trail_scale.x,
            trail_length,
            // Keep z scaling.
            current_trail_scale.z,
        ));
        // ANCHOR_END: on_start_begin

        // ANCHOR: effect_spawn
        if let Some(intersection) = intersections.first() {
            if let Some(effect) = self.impact_effect.as_ref() {
                effect.instantiate_at(
                    ctx.scene,
                    intersection.position.coords,
                    math::vector_to_quat(intersection.normal),
                );
            }
        }
        // ANCHOR_END: effect_spawn

        // ANCHOR: object_pushing
        if let Some(intersection) = intersections.first() {
            let collider = ctx.scene.graph.try_get(intersection.collider)?;
            let rigid_body_handle = collider.parent();
            let rigid_body = ctx
                .scene
                .graph
                .try_get_mut_of_type::<RigidBody>(rigid_body_handle)?;

            if let Some(force_dir) =
                (intersection.position.coords - this_node_position).try_normalize(f32::EPSILON)
            {
                let force = force_dir.scale(200.0);

                rigid_body.apply_force_at_point(force, intersection.position.coords);
                rigid_body.wake_up();
            }
        }
        // ANCHOR_END: object_pushing

        // ANCHOR: on_start_end
        Ok(())
    }
    // ANCHOR_END: on_start_end
}
