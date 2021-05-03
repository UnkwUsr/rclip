#!/bin/bash

if [[ -z "$FZF_PROMPT" ]]; then
    FZF_PROMPT="> "
fi

RCLIP_HOME="$HOME/.rclip"
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
' $(rg --sort path --files-with-matches .) | fzf --tac --no-sort -d : --with-nth 2.. --preview "cat {1}" --prompt "$FZF_PROMPT" $FZF_FLAGS | awk -F : '{print $1}')
#./UTF8_STRING/1619123453031

if [[ -z "$PICKED_FILE" ]]; then
    exit 1
fi

TARGET_NAME=$(dirname $PICKED_FILE)
FILE_NAME="$RCLIP_HOME/$PICKED_FILE"

