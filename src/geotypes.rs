use serde::Deserialize;

#[derive(Clone, PartialEq, Eq, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timezone {
    pub gmt_offset: i8,
    pub dst_offset: i8,
    pub time_zone_id: String,
}

#[derive(Clone, PartialEq, Eq, Debug, Default, Deserialize)]
pub struct AlternateName {
    pub name: String,
    pub lang: String,
}

#[derive(Clone, PartialEq, Eq, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Continent {
    pub lng: String,
    pub lat: String,
    pub geoname_id: u32,
    pub timezone: Timezone,
    pub toponym_name: String,
    pub ascii_name: String,
    pub name: String,
    pub continent_code: String,
    pub population: u32,
    #[serde(rename = "wikipediaURL")]
    pub wikipedia_url: String,
    pub alternate_names: Vec<AlternateName>,
    pub cc2: Option<String>,
}

#[derive(Clone, PartialEq, Debug, Default, Deserialize)]
pub struct City {
    pub geonameid: u32,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub countrycode: String,
    pub population: u32,
    pub timezone: String,
    pub admin1code: String,
    pub alternatenames: Vec<String>,
}

#[derive(Clone, PartialEq, Eq, Debug, Default, Deserialize)]
pub struct UsCounty {
    pub fips: String,
    pub name: String,
    pub state: String,
}

#[derive(Clone, PartialEq, Eq, Debug, Default, Deserialize)]
pub struct Country {
    pub geonameid: u32,
    pub name: String,
    pub iso: String,
    pub iso3: String,
    pub isonumeric: u32,
    pub fips: String,
    pub continentcode: String,
    pub capital: String,
    pub areakm2: u32,
    pub population: u32,
    pub tld: String,
    pub currencycode: String,
    pub currencyname: String,
    pub phone: String,
    pub languages: String,
    pub neighbours: String,
}

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct UsState {
    pub code: String,
    pub name: String,
    pub fips: String,
    pub geonameid: u32,
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use indoc::indoc;

    use super::*;

    #[test]
    fn deserialise_timezone() {
        let json = indoc! {r#"
        {
            "gmtOffset": 1,
            "timeZoneId": "Africa/Bangui",
            "dstOffset": 1
        }
        "#};

        let t: Timezone = serde_json::from_str(json).unwrap();
        assert_eq!(
            t,
            Timezone {
                gmt_offset: 1,
                time_zone_id: "Africa/Bangui".into(),
                dst_offset: 1
            }
        );
    }

    #[test]
    fn deserialise_alternate_name() {
        let json = indoc! {r#"
        {
            "name": "아프리카",
            "lang": "ko"
        }
        "#};

        let an: AlternateName = serde_json::from_str(json).unwrap();
        assert_eq!(
            an,
            AlternateName {
                name: "아프리카".into(),
                lang: "ko".into()
            }
        );
    }

    #[test]
    fn deserialise_continent() {
        let json = indoc! {r#"
        {
            "lng": "21.09375",
            "geonameId": 6255146,
            "timezone": {
                "gmtOffset": 1,
                "timeZoneId": "Africa/Bangui",
                "dstOffset": 1
            },
            "toponymName": "Africa",
            "asciiName": "Africa",
            "name": "Africa",
            "population": 1031833000,
            "wikipediaURL": "en.wikipedia.org/wiki/Africa",
            "alternateNames": [
                {
                    "name": "아프리카",
                    "lang": "ko"
                },
                {
                    "name": "Aafrika",
                    "lang": "et"
                }
            ],
            "continentCode": "AF",
            "cc2": "AO,BF,BI,BJ",
            "lat": "7.1881"
        }
        "#};

        let c: Continent = serde_json::from_str(json).unwrap();
        assert_eq!(
            c,
            Continent {
                lng: "21.09375".into(),
                lat: "7.1881".into(),
                geoname_id: 6255146,
                timezone: Timezone {
                    gmt_offset: 1,
                    dst_offset: 1,
                    time_zone_id: "Africa/Bangui".into()
                },
                toponym_name: "Africa".into(),
                ascii_name: "Africa".into(),
                name: "Africa".into(),
                population: 1031833000,
                wikipedia_url: "en.wikipedia.org/wiki/Africa".into(),
                continent_code: "AF".into(),
                cc2: Some("AO,BF,BI,BJ".into()),
                alternate_names: vec![
                    AlternateName {
                        name: "아프리카".into(),
                        lang: "ko".into()
                    },
                    AlternateName {
                        name: "Aafrika".into(),
                        lang: "et".into()
                    },
                ]
            }
        );
    }

    #[test]
    fn deserialise_us_county() {
        let json = indoc! {r#"
        {
            "fips": "01003",
            "name": "Baldwin County",
            "state": "AL"
        }
        "#};

        let usc: UsCounty = serde_json::from_str(json).unwrap();
        assert_eq!(
            usc,
            UsCounty {
                fips: "01003".into(),
                name: "Baldwin County".into(),
                state: "AL".into()
            }
        );
    }

    #[test]
    fn deserialise_city() {
        let json = indoc! {r#"
        {
            "3041563": {
                "geonameid": 3041563,
                "name": "Andorra la Vella",
                "latitude": 42.50779,
                "longitude": 1.52109,
                "countrycode": "AD",
                "population": 20430,
                "timezone": "Europe/Andorra",
                "admin1code": "07",
                "alternatenames": [
                    "ALV",
                    "Ando-la-Vyey",
                    "Andora"
                ]
            }
        }
        "#};

        let c: HashMap<String, City> = serde_json::from_str(json).unwrap();
        assert_eq!(
            c,
            HashMap::from([(
                "3041563".into(),
                City {
                    geonameid: 3041563,
                    name: "Andorra la Vella".into(),
                    latitude: 42.50779,
                    longitude: 1.52109,
                    countrycode: "AD".into(),
                    population: 20430,
                    timezone: "Europe/Andorra".into(),
                    admin1code: "07".into(),
                    alternatenames: vec!["ALV".into(), "Ando-la-Vyey".into(), "Andora".into()]
                }
            )])
        );
    }

    #[test]
    fn deserialise_country() {
        let json = indoc! {r#"
        {
            "AD": {
                "geonameid": 3041565,
                "name": "Andorra",
                "iso": "AD",
                "iso3": "AND",
                "isonumeric": 20,
                "fips": "AN",
                "continentcode": "EU",
                "capital": "Andorra la Vella",
                "areakm2": 468,
                "population": 84000,
                "tld": ".ad",
                "currencycode": "EUR",
                "currencyname": "Euro",
                "phone": "376",
                "postalcoderegex": "^(?:AD)*(\\d{3})$",
                "languages": "ca",
                "neighbours": "ES,FR"
            }
        }
        "#};

        let c: HashMap<String, Country> = serde_json::from_str(json).unwrap();
        assert_eq!(
            c,
            HashMap::from([(
                "AD".into(),
                Country {
                    geonameid: 3041565,
                    name: "Andorra".into(),
                    iso: "AD".into(),
                    iso3: "AND".into(),
                    isonumeric: 20,
                    fips: "AN".into(),
                    continentcode: "EU".into(),
                    capital: "Andorra la Vella".into(),
                    areakm2: 468,
                    population: 84000,
                    tld: ".ad".into(),
                    currencycode: "EUR".into(),
                    currencyname: "Euro".into(),
                    phone: "376".into(),
                    languages: "ca".into(),
                    neighbours: "ES,FR".into()
                }
            )])
        );
    }
}
