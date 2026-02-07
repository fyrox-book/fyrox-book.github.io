use fyrox::{
    graph::SceneGraph,
    gui::{
        core::pool::Handle,
        dock::{
            config::DockingManagerLayoutDescriptor, DockingManager, DockingManagerBuilder,
            TileBuilder, TileContent,
        },
        widget::WidgetBuilder,
        window::{WindowBuilder, WindowTitle},
        BuildContext, UserInterface,
    },
};

// ANCHOR: create_docking_manager
fn create_docking_manager(ctx: &mut BuildContext) -> Handle<DockingManager> {
    let top_window = WindowBuilder::new(WidgetBuilder::new())
        .with_title(WindowTitle::text("Top Window"))
        .build(ctx);
    let bottom_window = WindowBuilder::new(WidgetBuilder::new())
        .with_title(WindowTitle::text("Bottom Window"))
        .build(ctx);
    let root_tile = TileBuilder::new(WidgetBuilder::new())
        .with_content(TileContent::VerticalTiles {
            splitter: 0.5,
            tiles: [
                TileBuilder::new(WidgetBuilder::new())
                    // Note that you have to put the window into a separate tile, otherwise
                    // you'll get unexpected results.
                    .with_content(TileContent::Window(top_window))
                    .build(ctx),
                TileBuilder::new(WidgetBuilder::new())
                    .with_content(TileContent::Window(bottom_window))
                    .build(ctx),
            ],
        })
        .build(ctx);
    DockingManagerBuilder::new(
        WidgetBuilder::new()
            .with_child(root_tile)
            .with_width(500.0)
            .with_height(500.0),
    )
    .build(ctx)
}
// ANCHOR_END: create_docking_manager

// ANCHOR: save_layout
fn save_layout(
    ui: &UserInterface,
    docking_manager_handle: Handle<DockingManager>,
) -> Option<DockingManagerLayoutDescriptor> {
    ui.try_get(docking_manager_handle)
        .ok()
        .as_ref()
        .map(|docking_manager| docking_manager.layout(ui))
}
// ANCHOR_END: save_layout
