#!/usr/bin/env python
# -*- coding: utf-8 -*-
import csv
import json

cities = {}
fcsv = open("data/cities15000.txt", "r")
reader = csv.reader(fcsv, "excel-tab")
headers = next(reader)
for record in reader:
    (
        geonameid,
        name,
        asciiname,
        alternatenames,
        latitude,
        longitude,
        featureclass,
        featurecode,
        countrycode,
        cc2,
        admin1code,
        admin2code,
        admin3code,
        admin4code,
        population,
        elevation,
        dem,
        timezone,
        modificationdate,
    ) = record

    # required because used as key
    if not geonameid:
        continue

    cities[geonameid] = {
        "geonameid": int(geonameid),
        "name": name,
        "latitude": float(latitude),
        "longitude": float(longitude),
        "countrycode": countrycode,
        "population": int(population),
        "timezone": timezone,
        "admin1code": admin1code,
        "alternatenames": alternatenames.split(","),
    }

with open("src/cities.json", "w", encoding="utf-8") as f:
    json.dump(cities, f, separators=(",", ":"), ensure_ascii=False)
