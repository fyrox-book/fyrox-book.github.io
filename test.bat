clear
cd ../Fyrox
cargo build
cd ../fyrox-book.github.io
mdbook test -L ../Fyrox/target/debug/deps
