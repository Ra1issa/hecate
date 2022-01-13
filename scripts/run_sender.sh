cd ..

sender_phone=+16172991780
message=Hello
receiver_phone=+16174190472

cargo run --release --bin moderator
cargo run --release --bin sender $message

cd ../signal-cli
# cat ../hecate/data/mfrank.txt
echo "hello" | ./build/install/signal-cli/bin/signal-cli -u ${sender_phone} send ${receiver_phone}
