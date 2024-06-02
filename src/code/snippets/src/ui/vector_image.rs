use fyrox::gui::{
    core::{algebra::Vector2, pool::Handle},
    vector_image::{Primitive, VectorImageBuilder},
    widget::WidgetBuilder,
    BuildContext, UiNode, BRUSH_BRIGHT,
};

// ANCHOR: make_cross_vector_image
fn make_cross_vector_image(ctx: &mut BuildContext, size: f32, thickness: f32) -> Handle<UiNode> {
    VectorImageBuilder::new(
        WidgetBuilder::new()
            // Color of the image is defined by the foreground brush of the base widget.
            .with_foreground(BRUSH_BRIGHT),
    )
    .with_primitives(vec![
        Primitive::Line {
            begin: Vector2::new(0.0, 0.0),
            end: Vector2::new(size, size),
            thickness,
        },
        Primitive::Line {
            begin: Vector2::new(size, 0.0),
            end: Vector2::new(0.0, size),
            thickness,
        },
    ])
    .build(ctx)
}
// ANCHOR_END: make_cross_vector_image
