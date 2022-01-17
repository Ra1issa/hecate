sender_phone=+16172991780
receiver_phone=+16174190472

touch ../data/mfrank_received.txt
cd ../../signal-cli

echo "Receive Message"
./build/install/signal-cli/bin/signal-cli -u ${receiver_phone} receive > ../hecate/data/mfrank_received.txt

cd ../hecate/data
touch output.txt
sed -n '/Body: /,$p' mfrank_received.txt > output.txt
touch output2.txt
sed 's/Profile.*/Profile/' output.txt > output2.txt
sed -e "s/Body: //g" output2.txt > output.txt
sed -e "s/Profile//g" output.txt > output2.txt
rm output.txt
mv output2.txt mfrank_received.txt


echo "Receiver Check Message"
cargo run --release --bin receiver

echo "Moderator Inspect Message"
cargo run --release --bin moderator_inspect
