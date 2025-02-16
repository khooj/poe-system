#!/usr/bin/env bash
find /my/dir/ \( -type f -o -type d \) -printf "%P\n" | tar -czf mydir.tgz --no-recursion -C /my/dir/ -T -
