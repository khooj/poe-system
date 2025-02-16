#!/usr/bin/env bash

cat rust/domain/dist/mods.min.json | jq '.[] | {text: .text, stats: .stats}' | jq -sc '.' > rust/domain/dist/mods.extracted.json
