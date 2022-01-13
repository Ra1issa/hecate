cargo build --release
cd ../libsignal-client/java
cargo update
./gradlew :java:build
cp java/build/libs/signal-client-java-0.9.2.jar ../../signal-cli/lib/
cd ../../signal-cli
./gradlew build -x test
./gradlew installDist
./gradlew distTar
