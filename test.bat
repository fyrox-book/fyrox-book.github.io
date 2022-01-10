cd ../fyrox
cargo build
cd ../rg3d-book.github.io
mdbook test -L ../fyrox/target/debug/deps
