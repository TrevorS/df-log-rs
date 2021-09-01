#!/usr/bin/env python

import collections
import re
import json
import sys


def get_colors(filters_dat_file):
    colors = {}

    with open(filters_dat_file) as fd:
        filters = json.load(fd)

        for filter in filters.values():
            group = filter["group"]
            color = filter["color"].lower()

            if color == "#fff":
                color = "#ffffff"

            if color == "#ff0":
                color = "#ffff00"

            for category in filter["categories"].keys():
                colors[(group, category)] = color

    return colors


def get_filters(filters_file):
    regex = re.compile("\[(.+)\]\[(.*)\] (.+)")

    with open(filters_file) as f:
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
            {
                "group": group,
                "category": category,
                "expressions": expressions,
            }
            for (group, category), expressions in results.items()
        ]

        return formatted_filters


def main(filters_file, filters_dat_file, settings_file):
    colors = get_colors(filters_dat_file)
    filters = get_filters(filters_file)

    for filter in filters:
        key = (filter["group"], filter["category"])

        filter["color"] = colors.get(key, None)

    settings = {
        "general": {
            "gamelog_path": "./data/gamelog.txt",
        },
        "filters": filters,
    }

    with open(settings_file, "w") as s:
        json.dump(settings, s, indent=4)


if __name__ == "__main__":
    if len(sys.argv) < 4:
        print("example usage:")
        print("gen_settings.py filters.txt filters.dat settings.json")
    else:
        main(sys.argv[1], sys.argv[2], sys.argv[3])
