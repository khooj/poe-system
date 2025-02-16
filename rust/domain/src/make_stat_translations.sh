#!/usr/bin/env bash
cat stat_translations.json | jq -r '.[].English[0].string'