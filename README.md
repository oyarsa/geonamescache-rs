# Geonames Cache

A Rust/Python library that provides functions to retrieve names, ISO and FIPS
codes of continents, countries as well as US states and counties as
dictionaries. The country and city datasets also include population and
geographic data. This is a port of https://github.com/yaph/geonamescache.

Geonames data is obtained from [GeoNames](http://www.geonames.org/).

## Installation

    [dependencies]
    geonamescache = { git = "https://github.com/oyarsa/geonamescache-rs", rev = "<version>" }

Where `<version>` is a git tag, branch name or commit hash.

## Functions

Currently, geonamescache provides the following functions, that return
dictionaries with the requested data:

- get_continents()
- get_countries()
- get_us_states()
- get_cities()
- get_countries_by_names()
- get_us_states_by_names()
- get_cities_by_name(name)
- get_us_counties()

In addition, you can search for cities by name.

- search_cities('NAME')

This function returns a list of city records that match the given `NAME`. It searches
the `alternatenames` attribute for matches (case-insensitive).

## Mappers

The `mappers` module provides function(s) to map data properties. Currently, you
can create a mapper that maps country properties, e.g. the `name` property to
the `iso3` property, to do so you'd write the following code:


```rust
use geonamescache::mappers::country;
let name_to_iso3 = country(|c| (c.name, c.iso3));
let uk = name_to_iso3.get("united kingdom");
assert_eq!(uk, Some("GBR"));
```

## Contributing

1. Fork the repository on GitHub
2. Commit your changes to the **develop** branch
3. Write test(s) for any new feature
4. Push your changes and send a pull request

If you wish to build the data from scratch, run `make all` to download the data
files and covert them to JSON. This requires the `GEONAMES_USER` environment variable
to be defined (e.g., `GEONAMES_USER=foo make all`). You can create an account at
http://www.geonames.org/
