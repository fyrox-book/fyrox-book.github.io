clear
cd ..
cargo build
cd ..-book.github.io
mdbook test -L ../target/debug/deps
