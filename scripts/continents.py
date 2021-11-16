#!/usr/bin/env python
# -*- coding: utf-8 -*-
# Get continent data oincluding bounding boxes and centroids for and store it
# in json file keyed by continentCode.
import json
import os
import sys
from typing import Any, Dict

import requests

# see http://download.geonames.org/export/dump/readme.txt
continent_ids = [6255146, 6255147, 6255148, 6255149, 6255151, 6255150, 6255152]
url = "http://api.geonames.org/getJSON"
params = {
    "formatted": "true",
    "username": os.environ["GEONAMES_USER"],
    "geonameId": None,
}
continents = {}


def account_ok(j: Dict[str, Any]) -> None:
    if j.get(u"status", {}).get(u"value") == 10:
        print(j[u"status"][u"message"])
        sys.exit(1)


for geoid in continent_ids:
    params["geonameId"] = geoid
    resp = requests.get(url, params=params)

    cont = json.loads(resp.text)
    account_ok(cont)

    if resp.ok:
        continents[cont["continentCode"]] = cont


with open("src/continents.json", "w") as f:
    json.dump(continents, f)
