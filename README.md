![Termint Logo](https://raw.githubusercontent.com/mrhdias/termint/main/icons/hicolor/64x64/apps/termint.svg)

# Termint
Minimal Gtk4/Vte4 terminal emulator written in Rust, perfect for enjoying a cup of mint tea while working on your command line skills.

[![Rust](https://github.com/mrhdias/termint/actions/workflows/rust.yml/badge.svg)](https://github.com/mrhdias/termint/actions/workflows/rust.yml)

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
  -a, --app-id <ID>
          window application ID (termint)
  -d, --dir <PATH>
          Sets a custom settings directory
  -i, --init-settings
          Create the directory with the default settings if they do not exist
  -e, --execute <CMD>
          Execute the specified command (for compatibility with xterm -e)
  -L, --login-shell <PATH>
          start shell as a login shell
  -D, --working-directory <PATH>
          directory to start in (CWD)
  -w, --window-size-pixels <WIDTHxHEIGHT>
          initial width and height, in pixels
  -h, --help
          Print help
  -V, --version
          Print version
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

> [!TIP]
> How to fix the issue in Archlinux with accents on letters in Wayland/Labwc?
```
# Install fcitx
$ sudo pacman -S fcitx5 fcitx5-gtk fcitx5-qt fcitx5-configtool
# Edit the Labwc environment file
$ nano -w .config/labwc/environment
```
Add the following lines:
```
GTK_IM_MODULE=fcitx
QT_IM_MODULE=fcitx
XMODIFIERS=@im=fcitx
GDK_BACKEND=wayland
```
