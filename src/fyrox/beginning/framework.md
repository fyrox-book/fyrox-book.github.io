# Framework

The engine offers special framework to start making games as quickly as possible. It cares about engine initialization,
handles window events, calls every required methods and so on. In other words it helps you to get started as quickly
as possible without a need to put dozens lines of code to just create a window with a game loop.

The simplest app could be created with this code:

```rust,no_run
# extern crate fyrox;

use fyrox::{
    engine::Engine,
    engine::framework::prelude::*,
};

struct Game { }

impl GameState for Game {
    fn init(_engine: &mut Engine) -> Self where Self: Sized {
        Self { }
    }
}

fn main() {
    Framework::<Game>::new()
        .unwrap()
        .title("Simple")
        .run();
}
```

The "work-horse" here is the `GameState` trait, it offers some optional method that could be used depending on
your needs.

- `fn init(engine: &mut Engine) -> Self where Self: Sized` - should create the instance of your game. It accepts the engine instance as the first argument
  which gives you full access to the engine during the initialization.
- `fn on_tick(&mut self, engine: &mut Engine, dt: f32, control_flow: &mut ControlFlow)` - the game loop, it will
  be called at fixed 60 FPS rate allowing you to run your game logic. The method gives you full access to the engine
  so you're able to work it freely. The `dt` argument returns the amount of seconds that passed from the previous
  call. The last argument (`control_flow`) allowing you to change the execution flow, for example setting it to
  `ControlFlow::Exit` will force the game to quit.
- `fn on_ui_message(&mut self, engine: &mut Engine, message: UiMessage)` - the main function that listens events
  from the user interface and gives you the ability to react. You should handle your UI in here.
- `fn on_device_event(&mut self, engine: &mut Engine, device_id: DeviceId, event: DeviceEvent)` - the function that
  allows you to handle input from physical devices (mouse, keyboard, gamepads, etc.).
- `fn on_window_event(&mut self, engine: &mut Engine, event: WindowEvent)` - the function that allows you to do
  something when the window of the game receives an event from operating system. The variety of events is large, and
  everything depends on what you need. For example this method could be useful handle keyboard events.
- `fn on_exit(&mut self, engine: &mut Engine)` - the function that will be called right before your application
  about to shut down allowing you to do some clean up or some other actions.

As you can see it is very concise and simple, every method serves a particular purpose. The most important method is
`on_tick`, it the place where all your game logic will be updated. To demonstrate this, let's add simple animation:

```rust,no_run
# extern crate fyrox;
# use fyrox::{
#   core::color::{Color, Hsv},
#   engine::{framework::prelude::*, Engine},
#   event_loop::ControlFlow,
# };
struct Game {
    hue: f32,
}

impl GameState for Game {
    fn init(_engine: &mut Engine) -> Self
    where
        Self: Sized,
    {
        Self { hue: 0.0 }
    }

    // Implement a function that will update game logic and will be called at fixed rate of 60 Hz.
    fn on_tick(&mut self, engine: &mut Engine, dt: f32, _: &mut ControlFlow) {
        // Increase hue at fixed rate of 24 degrees per second.
        self.hue += 24.0 * dt;

        // Slowly change color of the window.
        engine
            .renderer
            .set_backbuffer_clear_color(Color::from(Hsv::new(self.hue % 360.0, 100.0, 100.0)))
    }
}
```

This piece of code will slowly change the background color of the window going through all colors of rainbow. 