#!/bin/bash

install -Dm755 scripts/copy.sh /usr/bin/rclip_copy
install -Dm755 scripts/rm.sh /usr/bin/rclip_rm

install -Dm755 scripts/feh_pick_image.sh /usr/share/rclip/feh_pick_image.sh
install -Dm755 scripts/fzf_pick.sh /usr/share/rclip/fzf_pick.sh
install -Dm755 scripts/_pick.sh /usr/share/rclip/_pick.sh
