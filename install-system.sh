#!/bin/sh
set -e

sudo mkdir -p /usr/share/icons/hicolor/22x22/apps
sudo mkdir -p /usr/share/krunner/dbusplugins
sudo mkdir -p /usr/share/dbus-1/services

cargo build --profile release
sudo cp target/release/krunner-cratesio /usr/bin

sudo cp icons/* /usr/share/icons/hicolor/22x22/apps/
sudo cp krunner-cratesio.desktop /usr/share/krunner/dbusplugins/
sed "s|@BINPATH@|$(which krunner-cratesio)|" net.heipiao.krunner-cratesio.service.in | sudo tee /usr/share/dbus-1/services/net.heipiao.krunner-cratesio.service > /dev/null

kquitapp6 krunner