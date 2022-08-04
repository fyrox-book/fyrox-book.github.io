# Logging

The engine has built-in logger that allows you to trace execution of your game by creating log entries when needed.

![Log](log.png)

The window allows you to select severity of the messages that will be put in the window:

- `Info+` will show all messages with `Info`, `Warning`, `Error` severities.
- `Warning+` will show all messages with `Warning` and `Error` severities.
- `Error` will show all messages with only `Error` severity.

Each log entry can be copied to the clipboard by right-clicking on it and pressing `Copy` in the context menu. You can
also clear the log using `Clear` button.

## Writing to the log

You can use one of `Log::info`, `Log::warn`, `Log::err` methods, or use `Log::writeln` with severity specified. It is also
possible to select desired severity level:

```rust,no_run
# extern crate fyrox;
# use fyrox::utils::log::{Log, MessageKind};
// These lines will be printed.
Log::info("This is some info");
Log::warn("This is some warning");
Log::err("This is some error");

Log::set_verbosity(MessageKind::Warning);

Log::info("This is some info"); // This won't be printed.
Log::warn("This is some warning");
Log::err("This is some error");
```