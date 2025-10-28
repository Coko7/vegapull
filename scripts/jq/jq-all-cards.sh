#!/usr/bin/env bash

packs_file=$(bash scripts/jq/locate-pack-json.sh)
lang_sub_dir=$(dirname "$packs_file")
jq --compact-output --slurp 'add' "$lang_sub_dir"/cards_*.json
