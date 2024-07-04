![Termint Logo](https://raw.githubusercontent.com/mrhdias/termint/main/icons/hicolor/64x64/apps/termint.svg)

# Termint
Minimal Gtk4/Vte4 terminal emulator written in Rust, perfect for enjoying a cup of mint tea while working on your command line skills.

To take a test drive:
```
$ git clone https://github.com/mrhdias/termint
$ cd termint
$ cargo build --release
$ scripts/install-icons.sh
$ target/release/termint -h
Minimal terminal emulator with mint flavor!
Usage: termint [OPTIONS]

Options:
  -d, --dir <DIRECTORY>  Sets a custom settings directory
  -i, --init             Create the directory with the default settings if they do not exist
  -c, --command <PATH>   Execute the specified command
  -h, --help             Print help
  -V, --version          Print version
```
> [!TIP]
> To create the initial directory with the default settings, the "-i" option must be passed.
```
$ target/release/termint -i
$ nano -w $HOME/.config/termint/termint.ini
$ nano -w $HOME/.config/termint/styles.css
$ cp target/release/termint $HOME/.local/bin
$ termint
```

Everyone Loves Screenshots!

![Termint Screenshot](https://raw.githubusercontent.com/mrhdias/termint/main/screenshot.png)
