# Progress bar

![progress bar](progress_bar.gif)

Progress bar is used to show a bar that fills in from left to right according to the progress value. It is used to
show progress for long actions.

## Example

```rust,no_run
{{#include ../code/snippets/src/ui/progress.rs:create_progress_bar}}
```

## Style

It is possible to specify custom indicator (the part that show the progress) and the back of
the progress bar. Use `ProgressBarBuilder::with_indicator` and `ProgressBarBuilder::with_body`
methods respectively. These methods can accept any widget, but usually it i a
`crate::border::Border`, `crate::image::Image`, `crate::nine_patch::NinePatch` widgets.

## Changing progress

To change progress of a progress bar all you need is to send `ProgressBarMessage::Progress` to it:

```rust,no_run
{{#include ../code/snippets/src/ui/progress.rs:change_progress}}
```