# Font

Font is used to store graphical representation of unicode characters. The engine supports TTF and OTF fonts, 
you can load pretty much any font from the internet and use it as is.

## Create New Font

There are two ways to create font instance - either load font from file, or load it directly from memory.

### Loading Font From File

Since every font in the engine is a resource, you can load fonts using standard resource manager like so:

```rust,no_run
{{#include ../code/snippets/src/ui/font.rs:load_font_from_file}}
```

### Creating Font From Memory

This option could be useful, if you already have your font loaded into memory. Loading font from data buffer is 
very simple:

```rust,no_run
{{#include ../code/snippets/src/ui/font.rs:load_font_from_memory}}
```

`data` input parameter could be a buffer that contains any valid TTF/OTF font. For example, you can load TTF file into
a data buffer and create font using the data buffer:

```rust ,no_run
{{#include ../code/snippets/src/ui/font.rs:load_font_from_file_memory}}
```

## Default Font

User interface provides its own font of fixed size, it is enough to cover most of the use cases, but the default font
includes only ASCII characters, if you need extended character set, you can replace default font using the following
code snippet:

```rust ,no_run,edition2018
{{#include ../code/snippets/src/ui/font.rs:set_default_font}}
```

## How to Change Font Size

All you need to do is to set font size in your Text or TextBox widgets like so: 

```rust ,no_run
{{#include ../code/snippets/src/ui/font.rs:text}}
```

You can also change the font size at runtime using `TextMessage::FontSize` message like so:

```rust ,no_run
{{#include ../code/snippets/src/ui/font.rs:set_font_size}}
```

## Important notes

Internally, font may use a number of texture atlases to pack all glyphs into a single texture. Font system creates
a new atlas for every glyph size. Each atlas could be split into multiple pages, which essentially just a texture
of a fixed size. Such paging is needed, because there's a hardware limit of maximum texture size on video cards and
instead of praying that everything fits into a single page, the engine automatically adds a new page if none of the
previous cannot fit a new character.

Keep in mind, that when you create a font, its atlases are empty. They're filled on demand when you try to use
a character that wasn't used previously. 