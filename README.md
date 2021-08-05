# Rclip

Rclip - clipboard manager written in rust. It's just a daemon which look for
clipboard updates and save them each per unique file.

## Features
* Each history entry saves in unique file.
* Easy to access each entry and write your own scripts to manipulate them.
* Easy to delete entries.
* Daemon does not handle clipboard history in RAM, so there is little memory consumption.
* Ability to set list of targets (in Xorg terms it means type of clipboard
  entry) that will be saved. (for example libreoffice formatted text, images, standard text).
* Ability to set minimal length of entry you want to save.
* Ability to pause rclip so it will not save next clipboard update (useful, for
  example, when setting password from password manager).
* Ability to use with fuzzy finders, like [fzf](https://github.com/junegunn/fzf) (script example presented).

## Installation

*Not yet provided. You can use `cargo install --path ./` and copy scripts to local bin folder*

## Usage

First thing you need to do - is run daemon:

```
rclip daemon
```

*(Recommended to add it to startup).*

All saved history entries stored in `~/.rclip/{target_name}/`, one file per entry.

### Copying and removing entries

For convenience you can use presented scripts `scripts/copy.sh` and
`scripts/rm.sh` or write your own.  Mentioned scripts by default operate with
text entries (using `fzf`), but you can pass argument `image` and it will
operate with images (using `feh`). To select image in feh just press "enter" key.

### Pause saving entries

If you use password manager, it will be useful to pause rclip, so just send
signal SIGUSR1 and rclip will skip next clipboard update:

```
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

