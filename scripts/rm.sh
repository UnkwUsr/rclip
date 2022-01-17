#!/bin/bash

RCLIP_HOME="${XDG_DATA_HOME:-$HOME/.local/share}/rclip"

export PICK_PURPOSE="rm"
export FZF_FLAGS="-m"

script_root=$(dirname "$0")/../share/rclip
source $script_root/_pick.sh

rm $PICKED_FILE
