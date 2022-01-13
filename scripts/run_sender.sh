cd ..

sender_phone=+16172991780
message=Hello
receiver_phone=+16174190472

echo "Generate Tokens"
cargo run --release --bin moderator_generate

echo "Generate Mfrank"
cargo run --release --bin sender $message

cd ../signal-cli
# cat ../hecate/data/mfrank.txt
# echo "hello"
echo "Send Mfrank"
cat ../hecate/data/mfrank.txt | ./build/install/signal-cli/bin/signal-cli -u ${sender_phone} send ${receiver_phone}
