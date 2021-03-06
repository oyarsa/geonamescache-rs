use std::collections::HashMap;

use maplit::{convert_args, hashmap};
use once_cell::sync::Lazy;

pub static COUNTRY_NAMES: Lazy<HashMap<String, &str>> = Lazy::new(|| {
    let mut m: HashMap<String, &str> = convert_args!(hashmap!(
        "Bolivia (Plurinational State of)"=> "Bolivia",
        "Bosnia-Herzegovina" => "Bosnia and Herzegovina",
        "Brunei Darussalam" => "Brunei",
        "Burma" => "Myanmar",
        "Cape Verde Islands" => "Cape Verde",
        "Czech Republic" => "Czechia",
        "China, Hong Kong SAR" => "Hong Kong",
        "China, Macao SAR" => "Macao",
        "Congo" => "Republic of the Congo",
        "Congo, Republic of the" => "Republic of the Congo",
        "Congo, Democratic Republic of" => "Democratic Republic of the Congo",
        "Congo, Democratic Republic of the" => "Democratic Republic of the Congo",
        "Congo DR" => "Democratic Republic of the Congo",
        "Cote d'Ivoire" => "Ivory Coast",
        "Côte d'Ivoire" => "Ivory Coast",
        "Democratic People's Republic of Korea" => "North Korea",
        "England" => "United Kingdom",
        "Federated States of Micronesia" => "Micronesia",
        "FYR Macedonia" => "Macedonia",
        "Hong Kong, SAR China" => "Hong Kong",
        "Hong Kong, China" => "Hong Kong",
        "Iran (I.R.)" => "Iran",
        "Iran (Islamic Republic of)" => "Iran",
        "Iran, Islamic Republic of" => "Iran",
        "Korea (Rep.)" => "South Korea",
        "Korea Republic" => "South Korea",
        "Korea, Democratic People's Republic of" => "North Korea",
        "Korea, North" => "North Korea",
        "Korea, Republic of" => "South Korea",
        "Korea, South" => "South Korea",
        "Lao People's Democratic Republic" => "Laos",
        "Libyan Arab Jamahiriya" => "Libya",
        "Macao, China" => "Macao",
        "Macau" => "Macao",
        "Macau, China" => "Macao",
        "Micronesia (Federated States of)" => "Micronesia",
        "Palestine" => "Palestinian Territory",
        "Palestinian Authority" => "Palestinian Territory",
        "Pitcairn Islands" => "Pitcairn",
        "Republic of Congo" => "Democratic Republic of the Congo",
        "Republic of Korea" => "South Korea",
        "Republic of Moldova" => "Moldova",
        "Réunion" => "Reunion",
        "Russian Federation" => "Russia",
        "Saint Barthélemy" => "Saint Barthelemy",
        "Saint Helena, Ascension and Tristan da Cunha" => "Saint Helena",
        "Slovak Republic" => "Slovakia",
        "State of Palestine" => "Palestinian Territory",
        "Syrian Arab Republic" => "Syria",
        "Taiwan Province of China" => "Taiwan",
        "Taiwan, China" => "Taiwan",
        "Tanzania, United Republic of" => "Tanzania",
        "TFYR Macedonia" => "Macedonia",
        "The Bahamas" => "Bahamas",
        "The former Yugoslav Republic of Macedonia" => "Macedonia",
        "the former Yugoslav Republic of Macedonia" => "Macedonia",
        "The Gambia" => "Gambia",
        "The Netherlands" => "Netherlands",
        "Timor-Leste" => "East Timor",
        "UAE" => "United Arab Emirates",
        "United Republic of Tanzania" => "Tanzania",
        "United Republic of Tanzania (the)" => "Tanzania",
        "United States Virgin Islands" => "U.S. Virgin Islands",
        "United States of America" => "United States",
        "US" => "United States",
        "USA" => "United States",
        "Vatican City" => "Vatican",
        "Venezuela (Bolivarian Republic of)" => "Venezuela",
        "Viet Nam" => "Vietnam",
        "West Bank and Gaza Strip" => "Palestinian Territory"
    ));

    let id: HashMap<_, _> = m.iter().map(|(k, &v)| (k.to_lowercase(), v)).collect();
    m.extend(id);
    m
});

pub static US_STATES: Lazy<HashMap<&str, HashMap<&str, &str>>> = Lazy::new(|| {
    hashmap! {
        "AK" => hashmap!{"code" => "AK", "name" => "Alaska", "fips" => "02", "geonameid" => "5879092"},
        "AL" => hashmap!{"code" => "AL", "name" => "Alabama", "fips" => "01", "geonameid" => "4829764"},
        "AR" => hashmap!{"code" => "AR", "name" => "Arkansas", "fips" => "05", "geonameid" => "4099753"},
        "AZ" => hashmap!{"code" => "AZ", "name" => "Arizona", "fips" => "04", "geonameid" => "5551752"},
        "CA" => hashmap!{"code" => "CA", "name" => "California", "fips" => "06", "geonameid" => "5332921"},
        "CO" => hashmap!{"code" => "CO", "name" => "Colorado", "fips" => "08", "geonameid" => "5417618"},
        "CT" => hashmap!{"code" => "CT", "name" => "Connecticut", "fips" => "09", "geonameid" => "4831725"},
        "DC" => hashmap!{"code" => "DC", "name" => "District of Columbia", "fips" => "11", "geonameid" => "4138106"},
        "DE" => hashmap!{"code" => "DE", "name" => "Delaware", "fips" => "10", "geonameid" => "4142224"},
        "FL" => hashmap!{"code" => "FL", "name" => "Florida", "fips" => "12", "geonameid" => "4155751"},
        "GA" => hashmap!{"code" => "GA", "name" => "Georgia", "fips" => "13", "geonameid" => "4197000"},
        "HI" => hashmap!{"code" => "HI", "name" => "Hawaii", "fips" => "15", "geonameid" => "5855797"},
        "IA" => hashmap!{"code" => "IA", "name" => "Iowa", "fips" => "19", "geonameid" => "4862182"},
        "ID" => hashmap!{"code" => "ID", "name" => "Idaho", "fips" => "16", "geonameid" => "5596512"},
        "IL" => hashmap!{"code" => "IL", "name" => "Illinois", "fips" => "17", "geonameid" => "4896861"},
        "IN" => hashmap!{"code" => "IN", "name" => "Indiana", "fips" => "18", "geonameid" => "4921868"},
        "KS" => hashmap!{"code" => "KS", "name" => "Kansas", "fips" => "20", "geonameid" => "4273857"},
        "KY" => hashmap!{"code" => "KY", "name" => "Kentucky", "fips" => "21", "geonameid" => "6254925"},
        "LA" => hashmap!{"code" => "LA", "name" => "Louisiana", "fips" => "22", "geonameid" => "4331987"},
        "MA" => hashmap!{"code" => "MA", "name" => "Massachusetts", "fips" => "25", "geonameid" => "6254926"},
        "MD" => hashmap!{"code" => "MD", "name" => "Maryland", "fips" => "24", "geonameid" => "4361885"},
        "ME" => hashmap!{"code" => "ME", "name" => "Maine", "fips" => "23", "geonameid" => "4971068"},
        "MI" => hashmap!{"code" => "MI", "name" => "Michigan", "fips" => "26", "geonameid" => "5001836"},
        "MN" => hashmap!{"code" => "MN", "name" => "Minnesota", "fips" => "27", "geonameid" => "5037779"},
        "MO" => hashmap!{"code" => "MO", "name" => "Missouri", "fips" => "29", "geonameid" => "4398678"},
        "MS" => hashmap!{"code" => "MS", "name" => "Mississippi", "fips" => "28", "geonameid" => "4436296"},
        "MT" => hashmap!{"code" => "MT", "name" => "Montana", "fips" => "30", "geonameid" => "5667009"},
        "NC" => hashmap!{"code" => "NC", "name" => "North Carolina", "fips" => "37", "geonameid" => "4482348"},
        "ND" => hashmap!{"code" => "ND", "name" => "North Dakota", "fips" => "38", "geonameid" => "5690763"},
        "NE" => hashmap!{"code" => "NE", "name" => "Nebraska", "fips" => "31", "geonameid" => "5073708"},
        "NH" => hashmap!{"code" => "NH", "name" => "New Hampshire", "fips" => "33", "geonameid" => "5090174"},
        "NJ" => hashmap!{"code" => "NJ", "name" => "New Jersey", "fips" => "34", "geonameid" => "5101760"},
        "NM" => hashmap!{"code" => "NM", "name" => "New Mexico", "fips" => "35", "geonameid" => "5481136"},
        "NV" => hashmap!{"code" => "NV", "name" => "Nevada", "fips" => "32", "geonameid" => "5509151"},
        "NY" => hashmap!{"code" => "NY", "name" => "New York", "fips" => "36", "geonameid" => "5128638"},
        "OH" => hashmap!{"code" => "OH", "name" => "Ohio", "fips" => "39", "geonameid" => "5165418"},
        "OK" => hashmap!{"code" => "OK", "name" => "Oklahoma", "fips" => "40", "geonameid" => "4544379"},
        "OR" => hashmap!{"code" => "OR", "name" => "Oregon", "fips" => "41", "geonameid" => "5744337"},
        "PA" => hashmap!{"code" => "PA", "name" => "Pennsylvania", "fips" => "42", "geonameid" => "6254927"},
        "RI" => hashmap!{"code" => "RI", "name" => "Rhode Island", "fips" => "44", "geonameid" => "5224323"},
        "SC" => hashmap!{"code" => "SC", "name" => "South Carolina", "fips" => "45", "geonameid" => "4597040"},
        "SD" => hashmap!{"code" => "SD", "name" => "South Dakota", "fips" => "46", "geonameid" => "5769223"},
        "TN" => hashmap!{"code" => "TN", "name" => "Tennessee", "fips" => "47", "geonameid" => "4662168"},
        "TX" => hashmap!{"code" => "TX", "name" => "Texas", "fips" => "48", "geonameid" => "4736286"},
        "UT" => hashmap!{"code" => "UT", "name" => "Utah", "fips" => "49", "geonameid" => "5549030"},
        "VA" => hashmap!{"code" => "VA", "name" => "Virginia", "fips" => "51", "geonameid" => "6254928"},
        "VT" => hashmap!{"code" => "VT", "name" => "Vermont", "fips" => "50", "geonameid" => "5242283"},
        "WA" => hashmap!{"code" => "WA", "name" => "Washington", "fips" => "53", "geonameid" => "5815135"},
        "WI" => hashmap!{"code" => "WI", "name" => "Wisconsin", "fips" => "55", "geonameid" => "5279468"},
        "WV" => hashmap!{"code" => "WV", "name" => "West Virginia", "fips" => "54", "geonameid" => "4826850"},
        "WY" => hashmap!{"code" => "WY", "name" => "Wyoming", "fips" => "56", "geonameid" => "5843591"}
    }
});
