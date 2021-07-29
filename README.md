# Rclip

Rclip - clipboard manager written in rust.

## Features
* Each history entry saves in unique file.
* Easy to access each entry and write your own scripts to manipulate them.
* Easy to delete entries.
* Ability to set list of targets (in Xorg terms it means type of clipboard.
  entry) that will be saved.  (for example libreoffice formatted text, images, standard text).
* Ability to use with fuzzy finders, like [fzf](https://github.com/junegunn/fzf) (script example presented).
* Setting clipboard entries is not handled by `rclip`. It's a work of different
  program, for example, `xclip`. So there is no errors maked in rclip :D.
* Daemon does not handle clipboard history in RAM, so there little RAM consumption.
* Ability to pause rclip so it will not save next clipboard update (useful, for
  example, when setting password from password manager)

## Installation

*Not yet provided. You can use `cargo install --path ./` and copy scripts to local bin folder*

## Usage

First thing you need to do - is run daemon:

`rclip daemon`

*(Recommended to add it to startup).*

All saved history entries stored in `~/.rclip/{target_name}/`, each file per entry.

For convenience you can use presented scripts `scripts/copy.sh` and `scripts/rm.sh` or write your own.

If you use password manager, it will be useful to pause rclip, so just send signal SIGUSR1 and rclip will skip next clipboard update:

`pkill -SIGUSR1 ^rclip$`

## Inspiration

Inspired by [greenclip](https://github.com/erebe/greenclip), a clipboard manager written in haskell.

