# Settings

Renderer has a large set of settings, that allows you to tweak graphics quality to find optimal balance between
rendering quality and performance. Quality settings are represented by the following structure:

```rust,no_run
# extern crate fyrox;
# use fyrox::renderer::{CsmSettings, ShadowMapPrecision};
struct QualitySettings {
    point_shadow_map_size: usize,
    point_soft_shadows: bool,
    point_shadows_enabled: bool,
    point_shadows_distance: f32,
    point_shadow_map_precision: ShadowMapPrecision,
    spot_shadow_map_size: usize,
    spot_soft_shadows: bool,
    spot_shadows_enabled: bool,
    spot_shadows_distance: f32,
    spot_shadow_map_precision: ShadowMapPrecision,
    csm_settings: CsmSettings,
    use_ssao: bool,
    ssao_radius: f32,
    light_scatter_enabled: bool,
    fxaa: bool,
    use_parallax_mapping: bool,
    use_bloom: bool,
}
```

- `point_shadow_map_size` - size of a cube map face of shadow map texture (in pixels). The higher, the better quality,
  but lower performance. Typical values for medium GPU (GTX 1050) is 1024 pixels.
- `point_soft_shadows` - should the shadows from point lights be smooth (`true`) or blocky (`false`). The latter option
  has better performance, but lower quality.
- `point_shadows_enabled` - are the shadows from point lights enabled? 
- `point_shadows_distance` - maximal distance from a camera to draw point light shadows. It is used to disable shadows
  on distant lights. The distance is given in meters. The lower the value, the better performance is.
- `point_shadow_map_precision` - defines bit-depth (`u16` or `u32`) for shadow map pixels. Lower bit depth means better
  performance and lower quality.
- `spot_shadow_map_size` - size of a shadow map texture for spotlights. The higher, the better quality,
  but lower performance. Typical values for medium GPU (GTX 1050) is 1024 pixels.
- `spot_soft_shadows` - should the shadows from spotlights be smooth (`true`) or blocky (`false`). The latter option
  has better performance, but lower quality.
- `spot_shadows_enabled` - are the shadows from spotlights enabled? 
- `spot_shadows_distance` - maximal distance from a camera to draw spotlight shadows. It is used to disable shadows
  on distant lights. The distance is given in meters. The lower the value, the better performance is. 
- `spot_shadow_map_precision` - defines bit-depth (`u16` or `u32`) for shadow map pixels.  Lower bit depth means better
  performance and lower quality.
- `csm_settings` - settings for cascaded shadow maps for directional lights.
  - `enabled` - whether cascaded shadow maps enabled or not. 
  - `size` - size of texture for each cascade.
  - `precision` - defines bit-depth (`u16` or `u32`) for shadow map pixels. Lower bit depth means better
    performance and lower quality.
  - `pcf` - should the shadows from directional lights be smooth (`true`) or blocky (`false`). The latter option
    has better performance, but lower quality.
- `use_ssao` - defines whether the renderer should perform separate screen-space ambient occlusion pass. This option
  has relatively small performance impact.
- `ssao_radius` - radius of sampling hemisphere used in SSAO, it defines much ambient occlusion will be in your scene.
  has no performance impact.
- `light_scatter_enabled` - global switch to enable or disable light scattering. Each light have its own scatter switch,
  but this one is able to globally disable scatter. Light scattering has medium performance impact, it also depends on 
  light count in your scene.
- `fxaa` - is full-screen anti-aliasing needed? This option has low performance impact.
- `use_parallax_mapping` - defines whether the renderer should use parallax mapping to simulate bumps and dents on
  flat surfaces using special textures. This option has low performance impact.
- `use_bloom` - defines whether the renderer should draw glowing pixels. This option has low performance impact.

## Presets

The renderer offers built-in presets for various graphics quality, use `QualitySettings::ultra()`, 
`QualitySettings::high()`, `QualitySettings::medium()` and `QualitySettings::low()` presets to quickly tune 
quality-performance balance.

## How to apply

To apply the settings, use `Renderer::set_quality_settings` method:

```rust,no_run
# extern crate fyrox;
# use fyrox::{
#     core::log::Log, engine::GraphicsContext, plugin::PluginContext, renderer::QualitySettings,
# };
# 
fn set_quality_settings(context: &mut PluginContext) {
    // Keep in mind, that graphics context can be non-initialized. This could happen if you're trying to access it before
    // your game received `Event::Resumed` event.
    if let GraphicsContext::Initialized(ref mut graphics_context) = context.graphics_context {
        let mut settings = QualitySettings::high();

        // Disable something.
        settings.use_ssao = false;
        settings.fxaa = false;

        // Apply.
        Log::verify(graphics_context.renderer.set_quality_settings(&settings))
    }
}
```

Keep in mind, that graphics context can be non-initialized. This could happen if you're trying to access it before your 
game received `Event::Resumed` event. See the docs for [Event::Resumed](https://docs.rs/fyrox/latest/fyrox/event/enum.Event.html#variant.Resumed) 
for more info. There is only one place, where graphics context is guaranteed to be initialized - 
`Plugin::on_graphics_context_initialized` method. Inside it, you can access the renderer by simple: 
`context.graphics_context.as_initialized_mut().renderer`, in other places you should always do a checked borrow.