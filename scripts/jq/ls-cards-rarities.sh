#!/usr/bin/env bash

bash scripts/jq/jq-all-cards.sh \
    | jq -r '.[].rarity' \
    | sort \
    | uniq -c \
    | sort -n
