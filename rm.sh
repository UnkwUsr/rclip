#!/bin/bash

FZF_PROMPT="rm> "
FZF_FLAGS="-m"

script_root=$(dirname $(realpath $(which $0)))
source $script_root/fzf_pick.sh

rm $FILE_NAME

