#!/bin/bash

RCLIP_HOME="$HOME/.rclip"

export PICK_PURPOSE="copy"

script_root=$(dirname $(realpath $(which $0)))
source $script_root/_pick.sh

# send a signal to rclip that now we will set entry from history
pkill -SIGUSR1 ^rclip$

# get current timestamp in milliseconds
NEW_FILE="$RCLIP_HOME/$TARGET_NAME/$(date +%s%3N)"
mv $PICKED_FILE $NEW_FILE

# nohup need to leave process running in the background (useful when call
# script by hotkey)
nohup xclip -t $TARGET_NAME -i $NEW_FILE -sel c > /dev/null 2> /dev/null
