# Fyrox Game Engine Guide Book

A cheat book for Fyrox Game Engine

Read the book [here](https://fyrox-book.github.io)

## Contributing

There are very simple contribution rules:

### Fixes and improvements for stable version

Create pull requests for merge into `main` branch, only if you're making some fixes for current stable version
from crates.io.

### Unstable version

If you're working on a feature for the engine and want to add a respective chapter or paragraph to the book, you
should create pull requests to `unstable` branch.

### Tests

Every code snippet is tested for compilation, please run `test.bat` (or its analog on Linux) before submitting
your work. Any fixes/improvements for Stable version should be tested with latest stable version of the engine,
other stuff should be tested with the current version of the engine from the repository.