#!/bin/bash

FZF_PROMPT="copy> "

script_root=$(dirname $(realpath $(which $0)))
source $script_root/fzf_pick.sh

# send a signal to rclip that now we will set entry from history
pkill -SIGUSR1 ^rclip$

# get current timestamp in milliseconds
NEW_FILE_NAME=$TARGET_NAME/$(date +%s%3N)
mv $FILE_NAME $NEW_FILE_NAME

# nohup need to leave process running in the background (useful when call
# script by hotkey)
nohup xclip -t $TARGET_NAME -i $NEW_FILE_NAME -sel c > /dev/null 2> /dev/null

