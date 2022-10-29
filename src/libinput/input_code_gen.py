#!/usr/bin/python3

import contextlib
from pprint import pprint
from typing import Generator, Iterable

try:
    import requests
except ImportError:
    print("Run `python3 -m pip install requests` to use this generator.")
    exit(1)

IEC_URL = "https://raw.githubusercontent.com/wayland-project/libinput/master/include/linux/linux/input-event-codes.h"
BANNED_KEYWORDS = ["RESERVED"]

res = requests.get(IEC_URL)

lines = res.text.splitlines()
doc: dict[str, dict[int, str]] = {}

def iter_lines(l: list) -> Generator:
    lines: Iterable[str] = iter(l)
    # get to start
    while True:
        try:
            line = next(lines)
        except StopIteration:
            print("Missing starting line.")
            exit(1)
        if "Device properties and quirks" in line:
            break
    # read file
    while True:
        try:
            line = next(lines)
        except StopIteration:
            break
        if line.startswith("#define"):
            line = line.removeprefix("#define ")
            segs = split_whitespace(line)
            group, name = segs[0].split("_", 1)
            base = 16 if "x" in segs[1] else 10
            with contextlib.suppress(ValueError):
                code = int(segs[1].split(" ")[0], base=base)
                yield group, name, code
        
def split_whitespace(raw: str) -> list[str]:
    whitespace = ["\t", " "]
    segments = []
    white = True
    for char in raw:
        if white and char not in whitespace:
                white = False
                segments.append(char)
        elif char in whitespace:
            white = True
        else:
            segments[-1] += char

    return segments

for group, name, code in iter_lines(lines):
    if name not in BANNED_KEYWORDS:
        if doc.get(group) is None:
            doc[group] = {}
        doc[group][code] = name
pprint(doc)