# Setting up

The user interface in the engine is extremely powerful, it allows you to build user interfaces of any complexity
with low effort. To start off we will use the simplest framework:

```rust,no_run
# extern crate rg3d;

use fyrox::{
    engine::{framework::prelude::*, Engine},
    gui::message::UiMessage
};

struct Game {
    // Empty for now
}

impl GameState for Game {
    fn init(engine: &mut Engine) -> Self
        where
            Self: Sized,
    {
        // Build context will be used in the next chapters.
        let ctx = &mut engine.user_interface.build_ctx();

        //
        // All widgets will be created here in the next chapters.
        //

        Self { }
    }

    fn on_ui_message(&mut self, engine: &mut Engine, message: UiMessage) {
        //
        // Here we'll handle messages from user interface.
        //
    }
}

fn main() {
    Framework::<Game>::new()
        .unwrap()
        .title("User Interface")
        .run();
}

```

This will be the basis for all next chapters. The most important places marked with respective comments.
