#!/usr/bin/env bash
cat stats.json | jq -r 'reduce (.result[].entries[] | { (.text): .id }) as $item ({}; . + $item)'
