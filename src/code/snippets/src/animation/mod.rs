use fyrox::{
    core::{reflect::prelude::*, type_traits::prelude::*, visitor::prelude::*},
    scene::{animation::spritesheet::SpriteSheetAnimation, dim2::rectangle::Rectangle},
    script::{ScriptContext, ScriptTrait},
};

// ANCHOR: animation
#[derive(Default, Clone, Debug, Reflect, Visit, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "aeebb95f-8e32-490e-971c-c22417bd19c5")]
#[visit(optional)]
struct Player {
    animation: SpriteSheetAnimation,
}

impl ScriptTrait for Player {
    fn on_update(&mut self, ctx: &mut ScriptContext) {
        // Update the animation first, it will switch current frame automatically if needed.
        self.animation.update(ctx.dt);

        if let Some(sprite) = ctx
            .scene
            .graph
            .try_get_mut(ctx.handle)
            .and_then(|n| n.cast_mut::<Rectangle>())
        {
            // Assign the texture from the animation to the sprite first.
            sprite
                .material()
                .data_ref()
                .set_texture(&"diffuseTexture".into(), self.animation.texture())
                .unwrap();

            // Set the current animation's UV rect to the sprite.
            sprite.set_uv_rect(self.animation.current_frame_uv_rect().unwrap_or_default());
        }
    }
}
// ANCHOR_END: animation
