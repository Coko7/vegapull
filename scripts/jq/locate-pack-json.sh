#!/usr/bin/env bash

# Locate packs.json
packs_file=$(fd packs.json --no-ignore-vcs --type file --max-depth 5 | head -n 1)
if [ ! -f "$packs_file" ]; then
    packs_file=$(fd . --no-ignore-vcs --type f --max-depth 5 | fzf)
    if [ ! -f "$packs_file" ]; then
        echo "no suitable packs file"
        exit 1
    fi
fi

echo "$packs_file"
