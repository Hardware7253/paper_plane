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

    cd builds

    # Copy program into the builds directory
    mkdir $name
    cp target/$target/release/$program_name $name
    cp -r ../assets $name

    # Zip
    zip -r $name.zip $name
    rm -r $name
}

target="x86_64-pc-windows-gnu"
name="windows"
program_name="paper_plane.exe"
build

target="x86_64-unknown-linux-gnu"
name="linux"
program_name="paper_plane"
build









#sed -i 's/^\(bevy = { version = "0.12.1", features = ["wayland"] } # Release\)$/#\1/' Cargo.toml #
#sed -i 's/^#\(bevy = { version = "0.12.1", features = ["dynamic_linking", "wayland"] } # Debug\)$/\1/' Cargo.toml