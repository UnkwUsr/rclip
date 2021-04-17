# Rclip

Rclip - clipboard manager written in rust.

## Features
* Save text clipboard history to file.
* Save images history per file.
* Print history list and pick what you want.
* Ability to set list of targets (in Xorg terms it means type of clipboard
  entry) that will be saved.  (for example libreoffice formatted text).
* Ability to use with fuzzy finders, like [fzf](https://github.com/junegunn/fzf) ([script](./fzf.sh) presented in the repository).
* Setting clipboard entries is not handled by `rclip`. It's a work of different
  program, for example, `xclip`. So there is no errors maked in rclip :D.
* Daemon does not handle clipboard history in RAM, so there little RAM consumption.

## Installation

*Not yet provided. You can use `cargo install --path ./` and copy `fzf.sh` to local bin folder*

## Usage

First thing you need to do - is run daemon:

`rclip daemon`

*(Recommended to add it to startup).*

Next, when you want to see your history and pick some entry, just use subcommand `list_and_set`:

`rclip list_and_set`

Or use presented [script](./fzf.sh) with fzf integration (preferable).

### About subcommand `list_and_set`:

It prints all list of clipboard history entries (with new lines replaced to spaces, for easy to view or use in fuzzy finders, like `fzf`).

List format:
```
ENTRY_INDEX ENTRY_FORMATTED_TEXT
```

... and then wait for input.
Input should be a number, index of entry which we want to get.
After program got input, it will print (this time to stderr (this was done for easy to use in scripts)) target_name (with `!` at start if real content of entry is stored in file) and on the next line original text of entry (or name of file where real entry data is stored).

Format:
```
[!(is stored in file)] TARGET_NAME
ENTRY_DATA/FILE_NAME
```

Also see `fzf.sh`, example of usage `rclip` with `fzf`.

## Inspiration

Inspired by [greenclip](https://github.com/erebe/greenclip), a clipboard manager written in haskell.

