#!/bin/bash

fnr() {
    sed -i "s/$old/$new/g" Cargo.toml
}



build() {

    # Disable dynamic linking for release builds
    old="dynamic_linking"
    new="wayland"
    fnr

    cargo build --target=$target --release

    # Enable dynamic linking for debug builds
    old="wayland"
    new="dynamic_linking"
    fnr

    # Copy program into the builds directory
    mkdir builds/$platform_name
    cp target/$target/release/$program_name builds/$platform_name/
    cp -r assets builds/$platform_name/

    # Zip
    cd builds
    zip -r $platform_name.zip $platform_name
    rm -r $platform_name
    cd ..
}

target="x86_64-pc-windows-gnu"
platform_name="windows"
program_name="paper_plane.exe"
build

target="x86_64-unknown-linux-gnu"
platform_name="linux"
program_name="paper_plane"
build









#sed -i 's/^\(bevy = { version = "0.12.1", features = ["wayland"] } # Release\)$/#\1/' Cargo.toml #
#sed -i 's/^#\(bevy = { version = "0.12.1", features = ["dynamic_linking", "wayland"] } # Debug\)$/\1/' Cargo.toml