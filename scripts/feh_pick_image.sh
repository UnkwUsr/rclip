#!/bin/bash

if [[ -z "$PICK_PURPOSE" ]]; then
    FEH_TITLE="_undefined purpose_"
else
    FEH_TITLE="$PICK_PURPOSE"
fi

TARGET_NAME="image/png"
PICKED_FILE=$(feh --reverse --title "$FEH_TITLE [%u of %l]" --action 'echo %F; kill %V' $RCLIP_HOME/$TARGET_NAME)

if [[ -z "$PICKED_FILE" ]]; then
    exit 1
fi
