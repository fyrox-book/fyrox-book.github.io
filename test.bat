cd ../fyrox
cargo build
cd ../fyrox-book.github.io
mdbook test -L ../fyrox/target/debug/deps
