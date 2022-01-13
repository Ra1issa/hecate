cd ..

sender_phone=+16172991780
message="Hello"
receiver_phone=+18573165383

cargo run --release --bin moderator
cargo run --release --bin sender $message

cd data
value=$(<mfrank.txt)

echo $value

cd ../../signal-cli
./gradlew run --args="-u ${sender_phone} send -m ${value} ${receiver_phone}"
