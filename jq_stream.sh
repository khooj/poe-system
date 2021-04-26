#!/usr/bin/env bash
jq -nc --stream 'fromstream(1|truncate_stream(inputs)) | select(.items[].influences != null) | .items[].influences' < <(cat test.json)
