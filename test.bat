cd ../rg3d
cargo build
cd ../rg3d-book.github.io
mdbook test -L ../rg3d/target/debug/deps
