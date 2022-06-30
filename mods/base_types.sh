#!/usr/bin/env bash
# from repoe
cat base_items.min.json | jq -r 'to_entries | .[] | .value.name' > base_types.txt