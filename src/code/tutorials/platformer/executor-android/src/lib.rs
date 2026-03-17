//! Android executor with your game connected to it as a plugin.
#![cfg(target_os = "android")]
use fyrox::{
    core::io, engine::executor::Executor, event_loop::EventLoopBuilder,
    platform::android::EventLoopBuilderExtAndroid,
};
use platformer::Game;

#[no_mangle]
fn android_main(app: fyrox::platform::android::activity::AndroidApp) {
    io::ANDROID_APP
        .set(app.clone())
        .expect("ANDROID_APP cannot be set twice.");
    #[allow(deprecated)]
    let event_loop = EventLoopBuilder::new()
        .with_android_app(app)
        .build()
        .unwrap();
    let mut executor = Executor::from_params(Some(event_loop), Default::default());
    executor.add_plugin(Game::default());
    executor.run()
}
