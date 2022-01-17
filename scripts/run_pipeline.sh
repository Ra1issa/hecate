message=Hello

echo "****************************"
echo "****************************"
echo "Moderator :: Generate Tokens"
cargo run --release --bin moderator_generate

echo "****************************"
echo "****************************"
echo "Sender :: Generate Mfrank"
cargo run --release --bin sender $message

echo "****************************"
echo "****************************"
echo "Platform :: Timestamp and Sign"
cargo run --release --bin platform

echo "****************************"
echo "****************************"
echo "Receiver :: Verify Message"
cargo run --release --bin receiver

echo "****************************"
echo "****************************"
echo "Moderaotr :: Verify Message and Trace"
cargo run --release --bin moderator_inspect
