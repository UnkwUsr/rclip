# Rclip

Rclip - clipboard manager written in rust. It's just a daemon which look for
clipboard updates and save them each per unique file.

## Features

* Each history entry saves in unique file.
* Daemon does not handle clipboard history in RAM, so there is little memory consumption.
* Checking for duplicates (and skipping them). P.S. it compares only last clipboard entry with current new.
* Ability to set list of targets (in Xorg terms it means type of clipboard
  entry) that will be saved. (for example libreoffice formatted text, images, standard text).
* Ability to set minimal length of entry you want to save.
* Ability to pause rclip so it will not save next clipboard update (useful, for
  example, when setting password from password manager).

### Bonus

* Easy to access each entry and write your own scripts to manipulate them.
* Easy to delete entries.
* Ability to use with fuzzy finders, like [fzf](https://github.com/junegunn/fzf) (scripts examples presented).

### Wayland support?

~~Native support is not planned as it seems to work nice through `xwayland`.
I'm finally migrated to wayland and it just works. I'm using `rclip` on daily
basis so going to fix problems with wayland if they occur.~~

I'm planning to add wayland support. It seemed to work nice through `xwayland`,
but turned out it is not (it does not detects clipboard change until one of
`xwayland` windows focused).

## Installation

### On Arch Linux

AUR package: [rclip-git](https://aur.archlinux.org/packages/rclip-git/)

### With cargo

```shell
cargo install rclipd
```

Also see [./install_scripts.sh](./install_scripts.sh) for installing provided
scripts.

## Usage

First thing you need to do - is run daemon:

```shell
rclip daemon
```

*(Recommended to add it to startup).*

All saved history entries stored in `~/.local/share/rclip/{target_name}/`
(where `~/.local/share` follows to $XDG_DATA_HOME by XDG specification), one file per entry.

### Copying and removing entries

For convenience you can use provided scripts `scripts/copy.sh` (or `rclip_copy` if installed from package) and
`scripts/rm.sh` (or `rclip_rm`) or write your own.  Mentioned scripts by default operate with
text entries (using `fzf`), but you can pass argument `image` and it will
operate with images (using `feh`). To select image in feh just press "enter" key.

Note: `feh` have default bind `ctrl+delete` which delete current file. ...And this work in `rclip_copy image`.

### Pause saving entries

If you use password manager, it will be useful to pause rclip, so just send
signal SIGUSR1 and rclip will skip next clipboard update:

```shell
pkill -SIGUSR1 ^rclip$
```

## Configuration

Config file `~/.config/rclip/config.toml` will be automatically created on first run.

There is only two settings:

1. `targets_list` - is a list of targets you want to save. Example (default):

    ```
    targets_list = [
      'image/png',
      'UTF8_STRING',
    ]
    ```

2. `min_length` - is a minimal length of entry you want to save. By default is `3`.

## Inspiration

Inspired by [greenclip](https://github.com/erebe/greenclip), a clipboard
manager written in haskell.
