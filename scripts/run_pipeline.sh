message=Hello

cargo run --release --bin moderator_generate
cargo run --release --bin sender $message
cargo run --release --bin receiver
cargo run --release --bin moderator_inspect
