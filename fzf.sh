#!/bin/bash

mkfifo /tmp/asdd
mkfifo /tmp/res_to_copy

# send a signal to rclip that now we will set entry from history
pkill -SIGUSR1 ^rclip$

# will wait until something will be printed to res_to_copy, and then pipe it to
# xclip. We need to do this BEFORE run rclip, because named pipe res_to_copy
# must be ready to read before we will write something to it.
copy_func() {
    read TARGET_NAME
    FILE_INPUT=""
    if [[ $TARGET_NAME == "! "* ]];
    then
        TARGET_NAME="${TARGET_NAME:2}"
        FILE_INPUT=~/.rclip/other_targets/$TARGET_NAME/$(cat)
    fi

    # nohup need to leave process running in the background (useful when call
    # script by hotkey)
    nohup xclip -i $FILE_INPUT -t $TARGET_NAME -sel c > /dev/null 2> /dev/null
}
copy_func < /tmp/res_to_copy &

# set asdd as stdin for rclip (which will be used for get picked index of entry)
# redirect rclip stderr to res_to_copy (which will be used for
#    getting original text entry)
# redirect rclip stdout to fzf
# redirect fzf stdout to asdd (which used for send index of picked entry to rclip)
rclip list_and_set < /tmp/asdd 2> /tmp/res_to_copy | fzf --tac --with-nth 2.. > /tmp/asdd

rm /tmp/asdd
rm /tmp/res_to_copy
