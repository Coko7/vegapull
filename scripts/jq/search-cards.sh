#!/usr/bin/env bash

query=$(gum input --prompt="Card name (fuzzy)> ")
[[ -z $query ]] && exit 1

results=$(bash scripts/jq/jq-fuzzy-name.sh "$query" | jq -r '.[] | "\(.pack_id)@\(.id)"')

packs_file=$(bash scripts/jq/locate-pack-json.sh)
lang_sub_dir=$(dirname "$packs_file")
images_dir="$lang_sub_dir/images"

echo "$results" | fzf --preview="scripts/jq/card-previewer.sh {} $images_dir" --height 80%
