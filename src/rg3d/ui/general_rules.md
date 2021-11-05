# General rules

This chapter covers general rules that will help you to write code that will be easy to understand.

## Fluent syntax

Widget builders supports fluent syntax, this means that you can create your widget in series of nested 
call of other widget builders. In code, it looks something like this:

```rust
# extern crate rg3d;
# use rg3d::{
#     core::pool::Handle,
#     engine::resource_manager::ResourceManager,
#     gui::{
#         button::ButtonBuilder, image::ImageBuilder, widget::WidgetBuilder, UiNode,
#         UserInterface,
#     },
#     utils::into_gui_texture,
# };
# fn create_fancy_button(ui: &mut UserInterface, resource_manager: ResourceManager) -> Handle<UiNode> {
# let ctx = &mut ui.build_ctx();
ButtonBuilder::new(WidgetBuilder::new())
    .with_back(
        ImageBuilder::new(WidgetBuilder::new())
            .with_texture(into_gui_texture(
                resource_manager.request_texture("path/to/your/texture", None),
            ))
            .build(ctx),
    )
    .with_text("Click me!")
    .build(ctx)
# }
```

This code snippet creates a button with an image and a text. Actually it creates **three** widgets, that forms 
complex hierarchy. The topmost widget in hierarchy is the `Button` widget itself, it has two children widgets:
background image and a text. Background image is set explicitly by calling image widget builder with specific 
texture. The text is created implicitly, the button builder creates `Text` widget for you and attaches it to 
the button. The structure of the button can contain _any_ amount of nodes, for example you can create a button
that contains text with some icon. To do that, replace `.with_text("My Button")` with this:

```rust
# extern crate rg3d;
# use rg3d::{
#     core::pool::Handle,
#     engine::resource_manager::ResourceManager,
#     gui::{
#         button::ButtonBuilder,
#         grid::{Column, GridBuilder, Row},
#         image::ImageBuilder,
#         text::TextBuilder,
#         widget::WidgetBuilder,
#         UiNode, UserInterface,
#     },
#     utils::into_gui_texture,
# };
# 
# fn create_fancy_button(
#     ui: &mut UserInterface,
#     resource_manager: ResourceManager,
# ) -> Handle<UiNode> {
#     let ctx = &mut ui.build_ctx();
# 
#     ButtonBuilder::new(WidgetBuilder::new())
        .with_content(
            GridBuilder::new(
                WidgetBuilder::new()
                    .with_child(
                        ImageBuilder::new(WidgetBuilder::new().on_column(0))
                            .with_texture(into_gui_texture(
                                resource_manager.request_texture("your_icon", None),
                            ))
                            .build(ctx),
                    )
                    .with_child(
                        TextBuilder::new(WidgetBuilder::new().on_column(1))
                            .with_text("My Button")
                            .build(ctx),
                    ),
            )
            .add_row(Row::stretch())
            .add_column(Column::auto())
            .add_column(Column::stretch())
            .build(ctx),
        )
#       .build(ctx)
# }
```

Quite often you need to store a handle to a widget in a variable, there is one neat trick to do that preserving
the fluent syntax:

```rust
# extern crate rg3d;
# use rg3d::{
#     core::pool::Handle,
#     engine::resource_manager::ResourceManager,
#     gui::{
#         button::ButtonBuilder, image::ImageBuilder, widget::WidgetBuilder, UiNode,
#         UserInterface,
#     },
#     utils::into_gui_texture,
# };
# fn create_fancy_button(ui: &mut UserInterface, resource_manager: ResourceManager) -> Handle<UiNode> {
# let ctx = &mut ui.build_ctx();
let image;
ButtonBuilder::new(WidgetBuilder::new())
    .with_back({
        image = ImageBuilder::new(WidgetBuilder::new())
            .with_texture(into_gui_texture(
                resource_manager.request_texture("path/to/your/texture", None),
            ))
            .build(ctx);
        image
    })
    .with_text("Click me!")
    .build(ctx)
# }
// image now contains a handle of the Image widget 
```

## Should I create a custom widget or use composition of other widgets?

The answer depends on the use case, but the general rules here is quite simple: 

- If your widget exist in a single instance, then there is no need to create a custom widget for it.
- If you need to create multiple instances of your widget, and each widget will carry some specific data, then you
definitely need a custom widget.

Custom widgets have some limitations that could be limiting, one of them is that custom widgets do not have 
access to your code, since they're "living" inside UI and know nothing about the "environment" where they're 
being used.