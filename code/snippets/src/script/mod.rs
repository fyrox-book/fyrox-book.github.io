use fyrox::{
    core::{
        log::Log, pool::Handle, reflect::prelude::*, uuid::Uuid, uuid_provider,
        visitor::prelude::*, TypeUuidProvider,
    },
    impl_component_provider,
    scene::node::Node,
    script::{ScriptContext, ScriptTrait},
};

// ANCHOR: access_other_1
#[derive(Clone, Debug, Reflect, Visit, Default)]
struct MyScript {
    second_node: Handle<Node>,
}

impl ScriptTrait for MyScript {
    // ANCHOR_END: access_other_1

    // ANCHOR: find_node
    fn on_start(&mut self, ctx: &mut ScriptContext) {
        self.second_node = ctx
            .scene
            .graph
            .find_by_name_from_root("SomeName")
            .map(|(handle, _)| handle)
            .unwrap_or_default();
    }
    // ANCHOR_END: find_node

    // ANCHOR: access_other_2
    fn on_update(&mut self, ctx: &mut ScriptContext) {
        if let Some(second_nodes_script_ref) = ctx
            .scene
            .graph
            .try_get_script_of::<MyOtherScript>(self.second_node)
        {
            if second_nodes_script_ref.counter > 60.0 {
                Log::info("Done!");
            }
        }

        // The code below is equivalent to the code above. The only difference is that
        // it borrows the node and then borrows the script from it, giving you access
        // to the node.
        if let Some(second_node_ref) = ctx.scene.graph.try_get(self.second_node) {
            if let Some(second_nodes_script_ref) = second_node_ref.try_get_script::<MyOtherScript>()
            {
                if second_nodes_script_ref.counter > 60.0 {
                    Log::info("Done!");
                }
            }
        }
    }

    fn id(&self) -> Uuid {
        Self::type_uuid()
    }
}

#[derive(Clone, Debug, Reflect, Visit, Default)]
struct MyOtherScript {
    counter: f32,
}

impl ScriptTrait for MyOtherScript {
    fn on_update(&mut self, _ctx: &mut ScriptContext) {
        // Counting.
        self.counter += 1.0;
    }

    fn id(&self) -> Uuid {
        Self::type_uuid()
    }
}
// ANCHOR_END: access_other_2

uuid_provider!(MyScript = "a9fb05ad-ab56-4be6-8a06-73e73d8b1f48");
impl_component_provider!(MyScript);

uuid_provider!(MyOtherScript = "a9fb05ad-ab56-4be6-8a06-73e73d8b1f49");
impl_component_provider!(MyOtherScript);
