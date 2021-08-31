import collections
import re
import json


def main():
    regex = re.compile("\[(.+)\]\[(.*)\] (.+)")

    with open("filters.txt", "r") as f:
        filters = f.readlines()
        results = collections.defaultdict(list)

        for filter in filters:
            if len(filter.strip()) == 0 or filter.startswith("#"):
                continue

            group, category, expression = regex.findall(filter)[0]
            expressions = results[(group, category)]
            expressions.append(expression.strip().replace('"', "").replace('\!', "!"))

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

        with open("generated_settings.json", "w") as s:
            json.dump(settings, s, indent=4)


if __name__ == "__main__":
    main()
