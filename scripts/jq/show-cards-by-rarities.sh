#!/usr/bin/env bash

all_cards=$(bash scripts/jq/jq-all-cards.sh)
all_rarities=$(echo "$all_cards" | jq -r '.[].rarity' | sort | uniq -c | sort -n)
rarity_pick=$(echo -e "$all_rarities" | fzf)
rarity=$(echo "$rarity_pick" | awk '{print $2}')

matched=$(echo -e "$all_cards" | jq -r --arg rarity "$rarity" '.[] | select(.rarity == $rarity) | "\(.pack_id)@\(.id)"')

packs_file=$(bash scripts/jq/locate-pack-json.sh)
lang_sub_dir=$(dirname "$packs_file")
images_dir="$lang_sub_dir/images"

echo "$matched" | fzf --preview="scripts/jq/card-previewer.sh {} $images_dir" --height 80%
