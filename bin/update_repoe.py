#!/usr/bin/env python3

import requests
import os

DATA = {
    "rust/domain/dist/base_items.min.json",
    "rust/domain/dist/stat_translations.min.json",
    "rust/domain/dist/stats.min.json",
    "rust/domain/dist/mods.min.json",
}

BASE_URL = "https://repoe-fork.github.io/"

if __name__ == '__main__':
    for data in DATA:
        basename = os.path.basename(data)
        r = requests.get(BASE_URL+basename)
        with open(data, "w+") as f:
            f.write(r.text)

