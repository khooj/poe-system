#!/usr/bin/env bash

cat domain/dist/mods.min.json | jq '.[] | {text: .text, stats: .stats}' | jq -sc '.' > domain/dist/mods.extracted.json
