sudo apt install -y libdbus-1-dev pkg-config
rustup toolchain install stable-x86_64-pc-windows-gnu

cargo build --release --verbose -j 2
cargo build --target x86_64-pc-windows-gnu --release --verbose -j 2

zip -j linux.zip target/release/beat_extractor
zip -j windows.zip target/x86_64-pc-windows-gnu/release/beat_extractor.exe