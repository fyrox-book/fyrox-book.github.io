use fyrox::core::color::Color;
use fyrox::gui::image::ImageBuilder;
use fyrox::gui::UiNode;
use fyrox::plugin::error::GameResult;
use fyrox::renderer::ui_renderer::UiRenderInfo;
use fyrox::{
    core::{algebra::Vector2, log::Log, pool::Handle},
    core::{reflect::prelude::*, visitor::prelude::*},
    engine::GraphicsContext,
    event::Event,
    gui::{button::ButtonBuilder, widget::WidgetBuilder, UserInterface},
    material::Material,
    plugin::{Plugin, PluginContext},
    resource::texture::{TextureResource, TextureResourceExtension},
    scene::Scene,
    utils,
};

// ANCHOR: rendering
#[derive(Default, Visit, Reflect, Debug)]
struct Game {
    // Add these fields to your game.
    my_ui: UserInterface,
    render_target: TextureResource,
    screen_size: Vector2<f32>,
}

impl Plugin for Game {
    fn init(&mut self, scene_path: Option<&str>, context: PluginContext) -> GameResult {
        // Add this code to your Plugin::init

        // Define the desired render target size.
        let width = 128;
        let height = 128;

        // Create render target and user interface.
        self.render_target = TextureResource::new_render_target(width, height);
        self.screen_size = Vector2::new(width as f32, height as f32);
        self.my_ui = UserInterface::new(self.screen_size);

        // Create some widgets as usual.
        ButtonBuilder::new(WidgetBuilder::new())
            .with_text("Click Me!")
            .build(&mut self.my_ui.build_ctx());

        // Use render_target as an ordinary texture - it could be applied to any material like so:
        let mut material = Material::standard();
        material.bind("diffuseTexture", Some(self.render_target.clone()));
        // This material **must** be assigned to some mesh in your scene!

        Ok(())
    }

    fn update(&mut self, context: &mut PluginContext) -> GameResult {
        // It is very important to update the UI every frame and process all events that
        // comes from it.
        self.my_ui
            .update(self.screen_size, context.dt, &Default::default());

        while let Some(message) = self.my_ui.poll_message() {
            // Do something with the events coming from the custom UI.
        }

        Ok(())
    }

    fn on_os_event(&mut self, event: &Event<()>, context: PluginContext) -> GameResult {
        // This is the tricky part. Event OS event handling will be different depending on the use case.
        // In cases if your UI just shows some information, this method can be fully removed. In case when
        // you need to interact with the UI, there are two different ways.
        // 1) If your UI will be incorporated in 2D scene, you need to patch mouse events - mostly positions
        // of the cursor so it will be in local coordinates.
        // 2) In 3D, it is much more complex - you need to patch mouse events as well, but use mouse OS events
        // to produce a picking ray and do an intersection test with a quad that will serve as a canvas for your
        // UI to obtain the local mouse coordinates.
        if let Event::WindowEvent { event, .. } = event {
            if let Some(event) = utils::translate_event(event) {
                self.my_ui.process_os_event(&event);
            }
        }

        Ok(())
    }

    fn before_rendering(&mut self, context: PluginContext) -> GameResult {
        // Render the UI before every other rendering operation, this way the texture will be ready for use immediately.
        if let GraphicsContext::Initialized(ref mut graphics_context) = context.graphics_context {
            Log::verify(graphics_context.renderer.render_ui(UiRenderInfo {
                ui: &self.my_ui,
                render_target: Some(self.render_target.clone()),
                clear_color: Color::TRANSPARENT,
                resource_manager: &context.resource_manager,
            }));
        }

        Ok(())
    }
}
// ANCHOR_END: rendering

// ANCHOR: reroute_scene_rendering
fn reroute_scene_rendering(
    width: u32,
    height: u32,
    scene: &mut Scene,
    context: &mut PluginContext,
) -> Handle<UiNode> {
    // Create render target first.
    let render_target = TextureResource::new_render_target(width, height);

    // Specify render target for the scene.
    scene.rendering_options.render_target = Some(render_target.clone());

    // The scene will be drawn in this image widget.
    ImageBuilder::new(
        WidgetBuilder::new()
            .with_width(width as f32)
            .with_height(height as f32),
    )
    .with_texture(render_target.clone())
    .build(&mut context.user_interfaces.first_mut().build_ctx())
}
// ANCHOR_END: reroute_scene_rendering
