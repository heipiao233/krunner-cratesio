#!/bin/sh
set -e

mkdir -p ~/.local/share/icons/hicolor/22x22/apps
mkdir -p ~/.local/share/krunner/dbusplugins
mkdir -p ~/.local/share/dbus-1/services

cargo install --path .

cp icons/* ~/.local/share/icons/hicolor/22x22/apps/
cp krunner-cratesio.desktop ~/.local/share/krunner/dbusplugins/
sed "s|@BINPATH@|$(which krunner-cratesio)|" io.github.heipiao233.krunner_cratesio.service.in > ~/.local/share/dbus-1/services/io.github.heipiao233.krunner_cratesio.service

kquitapp6 krunner
