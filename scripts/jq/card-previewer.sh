#!/usr/bin/env bash

card_identifier=$1
images_dir=$2

pack_id=$(echo "$card_identifier" | cut -d'@' -f1)
card_id=$(echo "$card_identifier" | cut -d'@' -f2)

bash "$SCRIPTS/fzf-preview.sh" "$images_dir/$pack_id/$card_id.png"
