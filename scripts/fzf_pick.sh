#!/bin/bash

if [[ -z "$PICK_PURPOSE" ]]; then
    FZF_PROMPT="_undefined purpose_> "
else
    FZF_PROMPT="$PICK_PURPOSE> "
fi

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
' $(rg --sort path --files-with-matches . $RCLIP_HOME) | fzf --tac -d : --with-nth 2.. --preview "cat {1}" --preview-window=wrap --prompt "$FZF_PROMPT" $FZF_FLAGS | awk -F : '{print $1}')

if [[ -z "$PICKED_FILE" ]]; then
    exit 1
fi

if [ $PICK_PURPOSE != "rm" ]; then
    TARGET_NAME=""
    temp_val=$(dirname $PICKED_FILE)
    until [ $temp_val = $RCLIP_HOME ]
    do
        TARGET_NAME="$(basename $temp_val)/$TARGET_NAME"
        temp_val=$(dirname $temp_val)
    done
    TARGET_NAME=${TARGET_NAME::-1}
fi
