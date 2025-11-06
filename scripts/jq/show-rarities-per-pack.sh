#!/usr/bin/env bash

source "scripts/bash-colors.sh"

bash scripts/jq/jq-rarities-per-pack.sh \
    | jq -c 'sort_by(
      [
        (.rarity_counts[] | select(.rarity=="SecretRare") | .count) // 0,
        (.rarity_counts[] | select(.rarity=="TreasureRare") | .count) // 0,
        (.rarity_counts[] | select(.rarity=="SuperRare") | .count) // 0,
        (.rarity_counts[] | select(.rarity=="Special") | .count) // 0
      ]
    ) | reverse[]' | while read -r pack; do 
    pack_id=$(echo "$pack" | jq -r '.pack_id')
    pack_name=$(bash scripts/jq/jq-all-packs.sh \
        | jq --arg id "$pack_id" -r '.[] | select(.id == $id) | .raw_title')

    secret_rare=$(echo "$pack" | jq '.rarity_counts[] | select(.rarity == "SecretRare") | .count' || echo 0)
    treasure_rare=$(echo "$pack" | jq '.rarity_counts[] | select(.rarity == "TreasureRare") | .count' || echo 0)
    super_rare=$(echo "$pack" | jq '.rarity_counts[] | select(.rarity == "SuperRare") | .count' || echo 0)
    special=$(echo "$pack" | jq '.rarity_counts[] | select(.rarity == "Special") | .count' || echo 0)
    leader=$(echo "$pack" | jq '.rarity_counts[] | select(.rarity == "Leader") | .count' || echo 0)
    rare=$(echo "$pack" | jq '.rarity_counts[] | select(.rarity == "Rare") | .count' || echo 0)
    uncommon=$(echo "$pack" | jq '.rarity_counts[] | select(.rarity == "Uncommon") | .count' || echo 0)
    common=$(echo "$pack" | jq '.rarity_counts[] | select(.rarity == "Common") | .count' || echo 0)

    all_count=$(echo "$pack" | jq '[.rarity_counts[].count] | add')

    gold_sum="${FG_YELLOW}${secret_rare:-0} 🌟 ${treasure_rare:-0}${COL_RESET}"
    purple_sum="${FG_PURPLE}${super_rare:-0} ⭐ ${special:-0}${COL_RESET}"
    pack_name="${FG_BLUE}$pack_name${COL_RESET}"

    echo -e "(${FG_RED}$pack_id${COL_RESET}) $gold_sum / $purple_sum $pack_name => $all_count"
done;
