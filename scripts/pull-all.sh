#!/usr/bin/env bash

LANGUAGE="japanese"
VEGA_DATA=data/$LANGUAGE

if [ -d "$VEGA_DATA" ]; then
    read -rp "The $VEGA_DATA is about to be wiped to hold the new data, do you want to proceed? (y/N) " confirm
    case $confirm in
        [Yy]* ) ;;
        * ) echo "Aborted" >&2; exit 1 ;;
    esac

    rm -rf $VEGA_DATA
fi

mkdir $VEGA_DATA
echo -e "Created dir: $VEGA_DATA\n"

echo "VegaPulling the list of packs ($LANGUAGE)..."

if ! ./target/release/vegapull --language $LANGUAGE packs > $VEGA_DATA/packs.json; then
    echo "Failed to pull list of packs using vegapull. Aborted" >&2
    exit 1
fi

count=$(jq length $VEGA_DATA/packs.json)

echo -e "Successfully pulled $count packs!\n"

function pull_cards() {
    local index=1
    local packs
    packs=$(cat $VEGA_DATA/packs.json)

    while read -r id; do
        echo -n "[$index/$count] VegaPulling cards for pack '$id'..."
        if ! ./target/release/vegapull --language $LANGUAGE cards "$id" > "$VEGA_DATA/cards_$id.json"; then
            echo "Failure"
            echo "Failed to pull cards using vegapull. Aborted" >&2
            return 1
        fi

        echo " OK"
        ((index++))
    done < <( echo "$packs" | jq -r '.[].id')

    echo "Successfully download data for $index packs!"
}

if ! pull_cards; then
    exit 1
fi

function download_images() {
    echo "NOT IMPLEMENTED YET"
}

read -rp "Download card images as well? (y/N) " confirm
case $confirm in
    [Yy]* ) download_images ;;
    * ) ;;
esac

echo "Successfully filled the punk records with latest data"
