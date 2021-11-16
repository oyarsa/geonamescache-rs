use std::sync::Mutex;
use std::{collections::HashMap, sync::Arc};

use crate::geotypes::{City, Continent, Country, UsCounty, UsState};

pub struct Geonamescache {
    us_states: HashMap<String, UsState>,
    continents: HashMap<String, Continent>,
    countries: HashMap<String, Country>,
    cities: HashMap<String, City>,
    us_counties: Vec<UsCounty>,
    cities_by_name_cache: Mutex<HashMap<String, Arc<HashMap<String, City>>>>,
}

macro_rules! load_data {
    ($path:literal) => {
        serde_json::from_str(include_str!($path))
            .expect(concat!("Internal error when loading file: ", $path))
    };
}

impl Geonamescache {
    pub fn new() -> Geonamescache {
        Self {
            us_states: load_us_states(),
            continents: load_data!("continents.json"),
            countries: load_data!("countries.json"),
            cities: load_data!("cities.json"),
            us_counties: load_data!("us_counties.json"),
            cities_by_name_cache: Mutex::new(HashMap::new()),
        }
    }

    pub fn get_continents(&self) -> &HashMap<String, Continent> {
        &self.continents
    }
    pub fn get_countries(&self) -> &HashMap<String, Country> {
        &self.countries
    }

    pub fn get_countries_by_names(&self) -> HashMap<&str, &Country> {
        self.get_countries()
            .values()
            .map(|d| (d.name.as_str(), d))
            .collect()
    }

    pub fn get_us_states(&self) -> &HashMap<String, UsState> {
        &self.us_states
    }

    pub fn get_cities(&self) -> &HashMap<String, City> {
        &self.cities
    }

    pub fn get_us_counties(&self) -> &Vec<UsCounty> {
        &self.us_counties
    }

    fn build_cities_by_name(&self, name: &str) -> HashMap<String, City> {
        self.get_cities()
            .iter()
            .filter(|(_, city)| city.name.to_lowercase() == name.to_lowercase())
            .map(|(gid, city)| (gid.clone(), city.clone()))
            .collect()
    }

    pub fn get_cities_by_name(&self, name: &str) -> Arc<HashMap<String, City>> {
        let mut cache = self.cities_by_name_cache.lock().unwrap();
        let m = cache
            .entry(name.to_string())
            .or_insert_with(|| Arc::new(self.build_cities_by_name(name)));
        Arc::clone(m)
    }

    pub fn search_cities(&self, query: &str) -> Vec<&City> {
        let query = query.to_lowercase();
        self.get_cities()
            .values()
            .filter(|geo| geo.alternatenames.iter().any(|x| x.to_lowercase() == query))
            .collect()
    }
}

impl<'a> Default for Geonamescache {
    fn default() -> Self {
        Self::new()
    }
}

pub fn load_us_states() -> HashMap<String, UsState> {
    crate::data::US_STATES
        .iter()
        .map(|(k, v)| {
            let v2 = UsState {
                code: v.get("name").unwrap().to_string(),
                name: v.get("name").unwrap().to_string(),
                fips: v.get("fips").unwrap().to_string(),
                geonameid: v.get("geonameid").unwrap().parse().unwrap(),
            };
            (k.to_string(), v2)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use more_asserts::assert_ge;
    use once_cell::sync::Lazy;

    use super::*;

    static GC: Lazy<Geonamescache> = Lazy::new(|| Geonamescache::new());

    #[test]
    fn test_continents() {
        let continents = GC.get_continents();
        let test_data = [
            ("AF", "Africa"),
            ("AN", "Antarctica"),
            ("AS", "Asia"),
            ("EU", "Europe"),
            ("NA", "North America"),
            ("OC", "Oceania"),
            ("SA", "South America"),
        ];

        for (code, name) in test_data {
            assert!(continents.contains_key(code));
            let c = continents.get(code).unwrap();
            assert_eq!(name, c.name);
        }

        for code in ["XX", "OO"] {
            assert!(!continents.contains_key(code));
        }
    }

    #[test]
    fn test_get_countries() {
        let countries = GC.get_countries();
        let test_data = [("ES", "Spain"), ("FR", "France"), ("US", "United States")];

        for (code, name) in test_data {
            assert!(countries.contains_key(code));
            let c = countries.get(code).unwrap();
            assert_eq!(name, c.name);
        }

        for code in ["XX", "OO"] {
            assert!(!countries.contains_key(code));
        }
    }

    #[test]
    fn test_us_states() {
        let us_states = GC.get_us_states();
        let test_data = [("NM", "New Mexico"), ("CA", "California"), ("NV", "Nevada")];

        for (code, name) in test_data {
            assert!(us_states.contains_key(code));
            let c = us_states.get(code).unwrap();
            assert_eq!(name, c.name);
        }

        for code in ["XX", "OO"] {
            assert!(!us_states.contains_key(code));
        }
    }

    #[test]
    fn test_get_countries_by_names() {
        let countries = GC.get_countries();
        let by_name = GC.get_countries_by_names();

        assert_eq!(countries.len(), by_name.len());
    }

    #[test]
    fn test_get_cities() {
        let cities = GC.get_cities();
        let test_data = [("3191316", "Samobor"), ("3107112", "Rivas-Vaciamadrid")];
        for (gid, name) in test_data {
            let c = cities.get(gid).unwrap();
            assert_eq!(name, c.name);
        }
    }

    #[test]
    fn test_get_cities_by_name_madrid() {
        let madrids = GC.get_cities_by_name("Madrid");
        assert_eq!(2, madrids.len());
    }

    #[test]
    fn test_cities_in_us_states() {
        let cities = GC.get_cities();
        let test_data = [("4164138", "Miami", "FL"), ("4525353", "Springfield", "OH")];
        for (gid, name, us_state) in test_data {
            let city = cities.get(gid).unwrap();
            let test_name = &city.name;
            let test_state = &city.admin1code;

            assert_eq!(name, test_name);
            assert_eq!(us_state, test_state);
        }
    }

    #[test]
    fn test_search_cities() {
        let city_names = ["Kiev", "kiev"];
        for name in city_names {
            let cities = GC.search_cities(name);
            assert_ge!(cities.len(), 1, "{}", name);
        }
    }

    #[test]
    fn test_us_counties_len() {
        let us_counties = GC.get_us_counties();
        assert_ge!(3234, us_counties.len());
    }
}
