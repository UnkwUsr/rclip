#!/bin/bash

RCLIP_HOME="$HOME/.rclip"

# send a signal to rclip that now we will set entry from history
# TODO: must be called exact before calling xclip
pkill -SIGUSR1 ^rclip$

cd $RCLIP_HOME

PICKED_FILE=$(gawk '
BEGIN {
    RS = ""
    FS ="\n"
}
BEGINFILE {
    printf("%s:", FILENAME);
};
{
    for (f=1; f<=NF; ++f) {printf("%s ", $f)};
};
ENDFILE {
    printf("\n")
};
' $(rg --sort path --files-with-matches .) | fzf --tac --no-sort -d : --with-nth 2.. | awk -F : '{print $1}')
#./UTF8_STRING/1619123453031

if [[ -z "$PICKED_FILE" ]]; then
    exit 1
fi

TARGET_NAME=$(dirname $PICKED_FILE)
FILE_NAME="$RCLIP_HOME/$PICKED_FILE"

# nohup need to leave process running in the background (useful when call
# script by hotkey)
nohup xclip -t $TARGET_NAME -i $FILE_NAME -sel c > /dev/null 2> /dev/null

