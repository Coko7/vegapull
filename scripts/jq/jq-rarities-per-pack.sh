#!/usr/bin/env bash

all_cards=$(bash scripts/jq/jq-all-cards.sh)
echo "$all_cards" | jq -r '
    [
        group_by(.pack_id)[] |
        {
            pack_id: .[0].pack_id,
            rarity_counts: (group_by(.rarity) | 
                map({
                    rarity: .[0].rarity,
                    count: length
                })
            )
        }
    ]
'
