pkill -f moderator_generate
pkill -f sender_fetch
pkill -f sender_send
pkill -f receiver
pkill -f moderator_inspect

message=Hello
echo "****************************"
echo "****************************"
echo "Moderator :: Generate Tokens"
cargo run --release --bin moderator_generate &
sleep 1s
echo "****************************"
echo "****************************"
echo "Sender :: Fetch Tokens"
cargo run --release --bin sender_fetch

echo "****************************"
echo "****************************"
echo "Sender :: Generate Mfrank"
cargo run --release --bin sender_send $message

echo "****************************"
echo "****************************"
echo "Platform :: Timestamp and Sign"
cargo run --release --bin platform

echo "****************************"
echo "****************************"
echo "Receiver :: Verify Message"
cargo run --release --bin receiver &
sleep 1s

echo "****************************"
echo "****************************"
echo "Moderator :: Verify Message and Trace"
cargo run --release --bin moderator_inspect
