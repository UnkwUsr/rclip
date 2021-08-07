#!/bin/bash

install -Dm755 scripts/copy.sh /usr/share/rclip/scripts/copy.sh
install -Dm755 scripts/feh_pick_image.sh /usr/share/rclip/scripts/feh_pick_image.sh
install -Dm755 scripts/fzf_pick.sh /usr/share/rclip/scripts/fzf_pick.sh
install -Dm755 scripts/_pick.sh /usr/share/rclip/scripts/_pick.sh
install -Dm755 scripts/rm.sh /usr/share/rclip/scripts/rm.sh

ln -s /usr/share/rclip/scripts/rm.sh /usr/bin/rclip_rm
ln -s /usr/share/rclip/scripts/copy.sh /usr/bin/rclip_copy
