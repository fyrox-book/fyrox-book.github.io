use fyrox::core::log::{Log, MessageKind};
use fyrox::core::{err, err_once, info, info_once, warn, warn_once};
use std::fs::File;

pub fn setup_logging() {
    // ANCHOR: setup_logging
    // It is possible to specify the file name for the logger output.
    Log::set_file_name("my_game.log");

    // The more precise control over the file can be achieved by creating
    // the log file manually:
    let file = File::create("my_game.log").unwrap();
    Log::set_file(Some(file));

    // The logging to a file can be disabled at any time by calling:
    Log::set_file(None);

    // By default, the logger also prints into stdout stream. This can
    // be disabled as well if not needed:
    Log::enable_writing_to_stdout(false);
    // ANCHOR_END: setup_logging
}

pub fn printing_to_the_log() {
    // ANCHOR: printing_to_the_log
    info!("This is some info. {}", 123);
    warn!("This is some warning. {}", 321);
    err!("This is some error. {}", 42);

    // The same can be achieved by direct function calls and format! macro.
    Log::info(format!("This is some info. {}", 123));
    Log::warn(format!("This is some warning. {}", 321));
    Log::err(format!("This is some error. {}", 42));
    // ANCHOR_END: printing_to_the_log
}

pub fn printing_once() {
    // ANCHOR: printing_once
    for _ in 0..10 {
        info_once!(0, "This info message will be printed only once.");
        warn_once!(1, "This warning message will be printed only once.");
        err_once!(2, "This error message will be printed only once.");
    }
    // ANCHOR_END: printing_once
}

pub fn set_verbosity() {
    // ANCHOR: set_verbosity
    // All these lines will be printed.
    info!("This is some info");
    warn!("This is some warning");
    err!("This is some error");

    Log::set_verbosity(MessageKind::Warning);

    info!("This is some info"); // This won't be printed.
    warn!("This is some warning");
    err!("This is some error");
    // ANCHOR_END: set_verbosity
}
