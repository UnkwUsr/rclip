#!/bin/bash

RCLIP_EXE="cargo run -q"

mkfifo asdd
mkfifo res_to_copy

# send a signal to rclip that now we will set entry from history
pkill -SIGUSR1 rclip

# will wait until something will be printed to res_to_copy, and then pipe it to
# xclip. We need to do that BEFORE run rclip, because named pipe res_to_copy
# must be ready to read before we will write something to it
xclip -sel c < res_to_copy &

# set asdd as stdin for rclip (that will be used for get picked index of entry)
# redirect rclip stderr to res_to_copy (that will be used for
#    getting original text entry)
# redirect rclip stdout to fzf
# redirect fzf stdout to asdd (that used for send index of picked entry to rclip)
$RCLIP_EXE list_and_set < asdd 2> res_to_copy | fzf --with-nth 2.. > asdd

rm asdd
rm res_to_copy
