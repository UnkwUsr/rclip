## [0.2.0] - 2022-01-17

### BREAKING CHANGES
- Follow XDG paths specification. Directory with clipboard history moved from
  `~/.rclip` to `~/.local/share/rclip` (means $XDG_DATA_HOME).

### Fixes
- Prevent infinity loop in scripts/fzf_pick.sh.
