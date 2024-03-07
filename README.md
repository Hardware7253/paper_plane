# Rust Paper Plane
![Cover](https://github.com/Hardware7253/paper_plane/assets/77563973/b5b31d1b-d180-4dba-ad48-2cc1f25cbc93)

Rust Paper Plane is an endless runner heavily inspired by the paper plane mini game from WarioWare, Inc.: Mega Microgames!

[Itch.io page](https://oxnh.itch.io/rust-paper-plane)

# Keybinds
Key           | Bind
------------- | -------------------------
Esacpe        | Pause / Unpause
A             | Steer left
D             | Steer right
F11           | Toggle fullscreen

# Troubleshooting

## Nvidia Optimus
If you have an NVIDIA Optimus setup on linux you may have some issues.
The game won't start because the GPU it selects is the one which your optimus software has disabled.
I use optimus-manager on my system, to solve this issue I set `pci_remove=yes` in `/etc/optimus-manager/optimus-manager.conf`
The solution should be similiar for alternative optimus software.

## Build Script
The build script builds for targets `x86_64-unknown-linux-gnu` & `x86_64-pc-windows-gnu` by default.

If a target is missing install it with `$ rustup target add`.

`mingw-w64` is required for cross compilation from linux to windows.
