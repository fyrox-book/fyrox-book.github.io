# Font

Font is used to store graphical representation of characters. The engine supports TTF and OTF fonts, you can load
pretty much any font from the internet and use it as is.

## Create New Font

There are two ways to create font instance - either load font from file, or load it directly from memory.

### Loading Font From File

The easiest way to create a font is load it from file:

```rust,no_run
# extern crate fyrox;
# use fyrox::gui::ttf::Font;
async fn load_font_from_file() -> Font {
    let character_set = Font::default_char_set(); // ASCII character set.
    Font::from_file("path/to/your/font.ttf", 20.0, character_set)
        .await
        .unwrap()
}
```

Please note, that this function is `async` due to the fact that it supports `fetch` API on WebAssembly, which does
remote file fetching which is asynchronous by its nature. If you don't plan to support WebAssembly and don't want to
use `async`, then the next section is for you.

### Creating Font From Memory

This option could be useful, if you already have your font loaded into memory. Loading font from data buffer is very
simple:

```rust,no_run
# extern crate fyrox;
# use fyrox::gui::ttf::Font;
fn load_font_from_memory(data: &[u8]) -> Font {
    let character_set = Font::default_char_set(); // ASCII character set.
    Font::from_memory(data, 20.0, character_set).unwrap()
}
```

`data` input parameter could be a buffer that contains any valid TTF/OTF font. For example, you can load TTF file into
a data buffer and create font using the data buffer:

```rust,no_run
# extern crate fyrox;
# use fyrox::gui::ttf::Font;
# use std::{fs::File, io::Read};
fn load_font_from_memory() -> Font {
    let character_set = Font::default_char_set(); // ASCII character set.

    let mut data = Vec::new();
    File::open("path/to/your/font.ttf")
        .unwrap()
        .read_to_end(&mut data)
        .unwrap();

    Font::from_memory(data, 20.0, character_set).unwrap()
}
```

## Default Font

User interface provides its own font of fixed size, it is enough to cover most of the use cases, but the default font
includes only ASCII characters, if you need extended character set, you can replace default font using the following
code snippet:

```rust,no_run,edition2018
# extern crate fyrox;
# use fyrox::gui::{ttf::Font, UserInterface};
async fn set_default_font(ui: &mut UserInterface) {
    // Select character set.
    let character_set = Font::korean_char_set();
    // Load font.
    let new_font = Font::from_file("path/to/your/font.ttf", 20.0, character_set)
        .await
        .unwrap();
    // Set as default font.
    ui.default_font.set(new_font)
}
```

## How to Change Font Size

Unfortunately, you need to create new font instance for that, there is a 
[tracking issue](https://github.com/FyroxEngine/Fyrox/issues/74) for that. Use any method from above paragraphs.

## Character Set

Current font implementation requires you to define fixed character set while creating an instance. What is character
set and how it can be extended? Character set is just a set of ranges from Unicode, for example here's korean character
set:

```rust,no_run
pub fn korean_char_set() -> &'static [Range<u32>] {
    &[
        // Basic Latin + Latin Supplement
        0x0020..0x00FF,
        // Korean alphabets
        0x3131..0x3163,
        // Korean characters
        0xAC00..0xD7A3,
        // Invalid
        0xFFFD..0xFFFD,
    ]
}
```

You can create custom character set yourself by defining desired ranges from Unicode.