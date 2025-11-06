#!/usr/bin/env bash

[ -z "$1" ] && echo "error: expects a query to be passed" && exit 1

bash scripts/jq/jq-all-cards.sh \
    | jq --arg name "$1" '[.[] | select(.name | test($name; "i"))]'
