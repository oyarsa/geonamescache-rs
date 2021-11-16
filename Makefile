all: dl tojson

data/cities15000.zip:
	mkdir -p data
	curl -o data/cities15000.zip http://download.geonames.org/export/dump/cities15000.zip

data/countryInfo.txt:
	mkdir -p data
	curl -o data/countryInfo.txt http://download.geonames.org/export/dump/countryInfo.txt

data/us_counties.txt:
	mkdir -p data
	curl -o data/us_counties.txt https://www2.census.gov/geo/docs/reference/codes/files/national_county.txt

data/cities15000.txt: data/cities15000.zip
	unzip -o data/cities15000.zip -d data
	rm data/cities15000.zip

dl: data/cities15000.txt data/countryInfo.txt data/us_counties.txt

tojson:
	'./scripts/continents.py'
	'./scripts/countries.py'
	'./scripts/cities.py'
	'./scripts/us_counties.py'

