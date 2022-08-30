import os
import re

patterns = dict()
for name in os.listdir("."):
    path = os.path.join(".", name)
    if os.path.isfile(path):
        with open(path) as file:
            program = file.read()
            for match in re.findall("(\[.*\])", program):
                if patterns.get(match):
                    patterns[match] += 1
                else:
                    patterns[match] = 1

filtered = filter(lambda entry: entry[1] != 1, patterns.items())
sorted = dict(sorted(filtered, key=lambda entry: entry[1], reverse=True)).keys()

print(sorted)
