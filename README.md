![Termint Logo](https://raw.githubusercontent.com/mrhdias/termint/main/icons/hicolor/64x64/apps/termint.svg)

# Termint
Minimal terminal emulator written in Rust with gtk4/vte4

To take a test drive:
```sh
$ git clone https://github.com/mrhdias/termint
$ cd termint
$ cargo build --release
$ scripts/install-icons.sh
$ target/release/termint
$ nano -w $HOME/.config/termint/termint.ini
$ nano -w $HOME/.config/termint/styles.css
$ cp target/release/termint $HOME/.local/bin
$ termint
```

Everyone Loves Screenshots!

![Termint Screenshot](https://raw.githubusercontent.com/mrhdias/termint/main/screenshot.png)
