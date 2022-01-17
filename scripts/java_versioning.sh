sudo update-alternatives --remove-all java
sudo update-alternatives --remove-all jar
sudo update-alternatives --remove-all jarsigner
sudo update-alternatives --remove-all javac
sudo update-alternatives --remove-all javadoc
sudo update-alternatives --remove-all javap


sudo update-alternatives --install "/usr/bin/java" "java" "/usr/lib/jvm/java-1.11.0-openjdk-amd64/bin/java" 10 \
    --slave "/usr/bin/jar"          "jar"           "/usr/lib/jvm/java-1.11.0-openjdk-amd64/bin/jar" \
    --slave "/usr/bin/jarsigner"    "jarsigner"     "/usr/lib/jvm/java-1.11.0-openjdk-amd64/bin/jarsigner" \
    --slave "/usr/bin/javac"        "javac"         "/usr/lib/jvm/java-1.11.0-openjdk-amd64/bin/javac" \
    --slave "/usr/bin/javadoc"      "javadoc"       "/usr/lib/jvm/java-1.11.0-openjdk-amd64/bin/javadoc" \
    --slave "/usr/bin/javap"        "javap"         "/usr/lib/jvm/java-1.11.0-openjdk-amd64/bin/javap"

sudo update-alternatives --install "/usr/bin/java" "java" "/usr/lib/jvm/java-1.17.0-openjdk-amd64/bin/java" 20 \
--slave "/usr/bin/jar"          "jar"           "/usr/lib/jvm/java-1.17.0-openjdk-amd64/bin/jar" \
--slave "/usr/bin/jarsigner"    "jarsigner"     "/usr/lib/jvm/java-1.17.0-openjdk-amd64/bin/jarsigner" \
--slave "/usr/bin/javac"        "javac"         "/usr/lib/jvm/java-1.17.0-openjdk-amd64/bin/javac" \
--slave "/usr/bin/javadoc"      "javadoc"       "/usr/lib/jvm/java-1.17.0-openjdk-amd64/bin/javadoc" \
--slave "/usr/bin/javap"        "javap"         "/usr/lib/jvm/java-1.17.0-openjdk-amd64/bin/javap"
