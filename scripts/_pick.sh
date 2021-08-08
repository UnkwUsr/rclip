#!/bin/bash

script_root=$(dirname "$0")/../share/rclip
if [ "$1" = "image" ]; then
    source $script_root/feh_pick_image.sh
else
    source $script_root/fzf_pick.sh
fi
