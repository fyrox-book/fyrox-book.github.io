# Button

![buttons](button.png)

## Simple button with text

To create a simple button with text you should do something like this:

```rust,no_run
{{#include ../code/snippets/src/ui/button.rs:create_button}}
```

How to create a button using custom dimensions (100x100) and custom text alignment (Vertical centered and Horizontal 
right aligned):

```rust,no_run
{{#include ../code/snippets/src/ui/button.rs:create_button_custom}}
```

## A button with image

More fancy-looking button with an image as a background could be created using this code snippet:

```rust,no_run
{{#include ../code/snippets/src/ui/button.rs:create_fancy_button}}
```

## Message handling

When clicked, a button sends a `ButtonMessage::Click` message, you can catch it in your code and do something
useful:

```rust,no_run
{{#include ../code/snippets/src/ui/button.rs:button_click_handling}}
```

## Using a button to exit the game

This example shows how to create a button that will close your game.

```rust,no_run
{{#include ../code/snippets/src/ui/button.rs:quit_button}}
```

