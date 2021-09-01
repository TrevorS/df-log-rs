#!/usr/bin/env python

import collections
import configparser
import re
import json
import sys


def should_skip_line(line):
    return len(line.strip()) == 0 or line.startswith("#")


def get_named_colors():
    return {
        "white": "#ffffff",
        "silver": "#c0c0c0",
        "gray": "#808080",
        "black": "#000000",
        "red": "#ff0000",
        "maroon": "#800000",
        "yellow": "#ffff00",
        "olive": "#808000",
        "lime": "#00ff00",
        "green": "#008000",
        "aqua": "#00ffff",
        "teal": "#008080",
        "blue": "#0000ff",
        "navy": "#000080",
        "fuchsia": "#ff00ff",
        "purple": "#800080",
    }


def get_highlights(highlights_file):
    regex = re.compile("\[.+\]\[(.+)\] (.+)")

    with open(highlights_file) as hf:
        highlights = hf.readlines()
        results = {}

        for highlight in highlights:
            if should_skip_line(highlight):
                continue

            color, words = regex.findall(highlight)[0]
            words = words.replace('"', "").split(",")

            for word in words:
                results[word] = color

        return results


def get_colors(filters_dat_file):
    with open(filters_dat_file) as fd:
        filters = json.load(fd)
        colors = {}

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


def get_icons(icons_file):
    cp = configparser.ConfigParser()
    cp.read(icons_file)

    results = {}

    for (icon, words) in cp["Icons"].items():
        words = words.split(",")

        for word in words:
            results[word] = icon

    return results


def get_filters(filters_file):
    regex = re.compile("\[(.+)\]\[(.*)\] (.+)")

    with open(filters_file) as f:
        filters = f.readlines()
        results = collections.defaultdict(list)

        for filter in filters:
            if should_skip_line(filter):
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


def main(filters_file, filters_dat_file, highlights_file, icons_file, settings_file):
    named_colors = get_named_colors()

    colors = get_colors(filters_dat_file)
    highlights = get_highlights(highlights_file)
    filters = get_filters(filters_file)
    icons = get_icons(icons_file)

    # populate filters with their corresponding colors
    for filter in filters:
        key = (filter["group"], filter["category"])

        filter["color"] = colors.get(key, None)

    settings = {
        "general": {
            "gamelog_path": "./data/gamelog.txt",
            "named_colors": named_colors,
        },
        "filters": filters,
        "highlights": highlights,
        "icons": icons,
    }

    with open(settings_file, "w") as s:
        json.dump(settings, s, indent=4)
        s.write("\n")


if __name__ == "__main__":
    if len(sys.argv) < 4:
        print("example usage:")
        print(
            "gen_settings.py filters.txt filters.dat wordcolor.txt icons.cfg settings.json"
        )
    else:
        main(sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4], sys.argv[5])
