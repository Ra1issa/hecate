mkdir -p ~/Documents/hecate/data/msgs
cp ../data/msgs/* ~/Documents/hecate/data/msgs

echo "****************************"
echo "****************************"
echo "Moderator :: Setup"
cargo run --release --bin moderator_setup

echo "****************************"
echo "****************************"
echo "Moderator :: Setup"
cargo run --release --bin platform_setup


echo "****************************"
echo "****************************"
echo "Generate Test Messages"
python3 generate_msg.py
echo "****************************"
echo "****************************"
