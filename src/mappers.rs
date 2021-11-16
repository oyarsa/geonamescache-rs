use std::collections::HashMap;

use crate::data::COUNTRY_NAMES;
use crate::geotypes::Country;
use crate::Geonamescache;

pub struct CountryMapper {
    dataset: HashMap<String, String>,
}

impl CountryMapper {
    pub fn get(&self, key: &str) -> Option<&str> {
        let key = key.to_lowercase();
        let key = COUNTRY_NAMES
            .get(key.as_str())
            .unwrap_or(&key.as_str())
            .to_string();
        self.dataset.get(&key).map(|x| x.as_str())
    }
}

/// Create a map between two fields in Country. This is done by taking a closure that
/// returns a (key, value) pair that will be converted to the map.
///
/// This map will be case-insensitive - the capitalisation of the key won't change
/// the result.
///
/// # Examples
/// ```
/// use geonamescache::mappers::country;
/// let name_to_iso3 = country(|c| (c.name, c.iso3));
/// let uk = name_to_iso3.get("united kingdom");
/// assert_eq!(uk, Some("GBR"));
/// ```
pub fn country<F>(f: F) -> CountryMapper
where
    F: Fn(Country) -> (String, String),
{
    let mut d: HashMap<_, _> = Geonamescache::new()
        .get_countries()
        .values()
        .cloned()
        .map(f)
        .collect();
    let id: HashMap<_, _> = d
        .iter()
        .map(|(k, v)| (k.to_lowercase(), v.clone()))
        .collect();
    d.extend(id);

    CountryMapper { dataset: d }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::COUNTRY_NAMES;

    #[test]
    fn test_mappings() {
        assert_eq!(COUNTRY_NAMES.get("Macau"), Some(&"Macao"));
        assert_eq!(COUNTRY_NAMES.get("Pitcairn Islands"), Some(&"Pitcairn"));
        assert_eq!(
            COUNTRY_NAMES.get("Saint Barthélemy"),
            Some(&"Saint Barthelemy")
        );
    }

    #[test]
    fn test_mappings_insensitive() {
        assert_eq!(COUNTRY_NAMES.get("macau"), Some(&"Macao"));
        assert_eq!(COUNTRY_NAMES.get("pitcairn islands"), Some(&"Pitcairn"));
        assert_eq!(
            COUNTRY_NAMES.get("saint barthélemy"),
            Some(&"Saint Barthelemy")
        );
    }

    #[test]
    fn test_country_name_iso3_mapper() {
        let mapper = country(|c| (c.name, c.iso3));
        let map = |input| mapper.get(input).unwrap();

        assert_eq!(map("Burma"), "MMR");
        assert_eq!(map("South Korea"), "KOR");
        assert_eq!(map("The Netherlands"), "NLD");
        assert_eq!(map("USA"), "USA");
    }
}
