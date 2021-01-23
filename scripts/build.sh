#!/usr/bin/bash

# NOTE current dir is autolink-root
cargo build --bin autolink --release
rm autolink
mv ./target/release/autolink .
chmod +x autolink