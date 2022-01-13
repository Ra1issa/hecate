cd ..

sender_phone=+16172991780
message=Hello
receiver_phone=+16174190472

cd ../signal-cli
cat ../hecate/data/mfrank.txt | ./build/install/signal-cli/bin/signal-cli -u ${receiver_phone} receive
