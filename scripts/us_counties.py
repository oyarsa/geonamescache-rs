#!/usr/bin/env python
# coding: utf-8
import json
import csv

counties = []

with open("data/us_counties.txt", "r", encoding="utf-8") as f:
    r = csv.reader(f)
    headers = next(r)
    for line in r:
        counties.append({"fips": line[1] + line[2], "name": line[3], "state": line[0]})

with open("src/us_counties.json", "w") as f:
    json.dump(counties, f)
