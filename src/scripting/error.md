# Error Handling

Pretty much every method of a plugin or a script returns a special type `GameResult` which is a wrapper over 
`Result<(), GameError>`. This allows you to easily handle various errors that may occur during the code execution by
applying `?` operator.

```rust
{{#include ../code/snippets/src/error/mod.rs:error_handling}}
```

When an error occurs in any of the methods, the engine simply prints it to the log and continues execution as usual.
This is the key difference between errors and standard panic mechanism.

The `GameError` type can hold errors of pretty much any kind, so any error that implements `std::error::Error` trait
can be returned.

## Backtrace capture

By default, all errors that may occur during the code execution don't capture the backtrace, which may significantly
complicate tracking of the original source of error. Backtrace capture can be enabled by using `enable_backtrace_capture`:
method.

```rust,no_run
{{#include ../code/snippets/src/error/mod.rs:enable_backtrace_capture}}
```

This way the engine will print the error message alongside with the backtrace which points to the exact place where the
error originates from.