#!/bin/bash

RCLIP_HOME="$HOME/.rclip"

export PICK_PURPOSE="rm"
export FZF_FLAGS="-m"

script_root=$(dirname $(realpath $(which $0)))
source $script_root/_pick.sh

rm $PICKED_FILE
