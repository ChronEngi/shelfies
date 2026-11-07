cargo run --release
rm shelfies.AppDir/urs/bin/shelfies
cp target/release/shelfies shelfies.AppDir/urs/bin
ARCH=x86_64 ./appimagetool-x86_64.AppImage shelfies.AppDir