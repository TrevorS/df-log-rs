#!/usr/bin/env python

import collections
import re
import json
import sys


def main(filters_file, settings_file):
    regex = re.compile("\[(.+)\]\[(.*)\] (.+)")

    with open(filters_file, "r") as f:
        filters = f.readlines()
        results = collections.defaultdict(list)

        for filter in filters:
            # ignore blank lines or comments
            if len(filter.strip()) == 0 or filter.startswith("#"):
                continue

            group, category, expression = regex.findall(filter)[0]

            # filter out spam settings for now
            if "spam" in (group, category):
                continue

            expressions = results[(group, category)]
            expressions.append(expression.strip().replace('"', "").replace("\!", "!"))

        formatted_filters = [
            {"group": group, "category": category, "expressions": expressions}
            for (group, category), expressions in results.items()
        ]

        settings = {
            "general": {
                "gamelog_path": "./data/gamelog.txt",
            },
            "filters": formatted_filters,
        }

        with open(settings_file, "w") as s:
            json.dump(settings, s, indent=4)


if __name__ == "__main__":
    if len(sys.argv) < 3:
        print("example usage:")
        print("gen_settings.py filters.txt settings.json")
    else:
        main(sys.argv[1], sys.argv[2])
