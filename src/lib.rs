#[macro_use]
extern crate lazy_static;
extern crate csv;
extern crate rustc_serialize;
extern crate itertools;
#[macro_use]
extern crate assert_approx_eq;

use itertools::{Itertools, Either};

use std::fmt;
use std::convert::From;

#[derive(RustcDecodable)]
struct Record {
    city: String,
    _city_ascii: String,
    latitude: f64,
    longitude: f64,
    population: f64,
    _country: String,
    _iso2: String,
    _iso3: String,
    _province: String,
}

#[derive(PartialEq, Clone, Debug)]
struct City {
    name: String,
    latitude: f64,
    longitude: f64,
    population: f64,
}

impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl From<Record> for City {
    fn from(record: Record) -> City {
        City {
            name: record.city,
            latitude: record.latitude,
            longitude: record.longitude,
            population: record.population,
        }
    }
}

lazy_static! {
    static ref DATA: Vec<City> = csv::Reader::from_file("./simplemaps-worldcities-basic.csv")
        .unwrap()
        .decode::<Record>()
        .map(Result::unwrap)
        .map(Record::into)
        .collect();

    static ref NORTH_POLE: City = City {
        name: String::from("North Pole"),
        latitude: 90.0,
        longitude: 0.0,
        population: 0.0,
    };

    static ref SOUTH_POLE: City = City {
        name: String::from("South Pole"),
        latitude: -90.0,
        longitude: 0.0,
        population: 0.0,
    };
}

const LATITUDE_TOLERANCE: f64 = 0.5;

fn same_latitude(lat: f64) -> Vec<City> {
    DATA
        .iter()
        .cloned()
        .filter(|city| {
            city.latitude < lat + LATITUDE_TOLERANCE &&
            lat - LATITUDE_TOLERANCE < city.latitude
        })
        .collect()
}

fn opposite_longitude(long: f64) -> f64 {
    let mut opposite_long = 180.0 - long.abs();

    if long > 0.0 {
        opposite_long *= -1.0;
    };

    opposite_long
}

const LONGITUDE_TOLERANCE: f64 = 0.5;

fn same_longitude(long: f64) -> Vec<City> {
    let opposite_long = opposite_longitude(long);

    DATA
        .iter()
        .cloned()
        .filter(|city| {
            (city.longitude < long + LONGITUDE_TOLERANCE &&
            long - LONGITUDE_TOLERANCE < city.longitude) ||
            (city.longitude < opposite_long + LONGITUDE_TOLERANCE &&
            opposite_long - LONGITUDE_TOLERANCE < city.longitude)
        })
        .collect()
}

fn top_10_by_population(mut cities: Vec<City>) -> Vec<City> {
    cities.sort_by(|a, b| b.population.partial_cmp(&a.population).unwrap());
    cities.into_iter().take(10).collect()
}

fn sort_easterly(mut cities: Vec<City>, start_long: f64) -> Vec<City> {
    cities.sort_by(|a, b| a.longitude.partial_cmp(&b.longitude).unwrap());

    let (mut west, mut east): (Vec<_>, Vec<_>) = cities
        .into_iter()
        .partition_map(|city| {
            if city.longitude <= start_long {
                Either::Left(city)
            } else {
                Either::Right(city)
            }
        });
    east.append(&mut west);
    east
}

fn sort_northerly(mut cities: Vec<City>, start_lat: f64, start_long: f64) -> Vec<City> {
    let start_long_negative = start_long < 0.0;

    let (mut same_side, mut opp_side): (Vec<_>, Vec<_>) = cities
        .into_iter()
        .partition_map(|city| {
            if start_long_negative {
                if city.longitude < 0.0 {
                   Either::Left(city)
                } else {
                    Either::Right(city)
                }
            } else {
                if city.longitude < 0.0 {
                   Either::Right(city)
                } else {
                    Either::Left(city)
                }
            }
        });

    same_side.sort_by(|a, b| a.latitude.partial_cmp(&b.latitude).unwrap());

    let (mut north, mut south): (Vec<_>, Vec<_>) = same_side
        .into_iter()
        .partition_map(|city| {
            if city.latitude > start_lat {
                Either::Left(city)
            } else {
                Either::Right(city)
            }
        });

    opp_side.sort_by(|a, b| b.latitude.partial_cmp(&a.latitude).unwrap());

    north.push(NORTH_POLE.clone());
    north.append(&mut opp_side);
    north.push(SOUTH_POLE.clone());
    north.append(&mut south);
    north
}

fn latitude_cities(latitude: f64, longitude: f64) -> Vec<City> {
    let cities = same_latitude(latitude);
    let cities = top_10_by_population(cities);
    let cities = sort_easterly(cities, longitude);
    cities
}

fn latitude_text(latitude: f64, longitude: f64) -> String {
    format!("If you fly along this latitude in an easterly direction, you will look down on {}.", latitude_cities(latitude, longitude).iter().join(", "))
}

fn decimal_to_degrees_minutes(coord: f64) -> (f64, f64) {
    (
        coord.abs().floor(),
        ((coord.abs() * 60.0) % 60.0).floor()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_finds_cities_with_same_latitude() {
        let lat = 40.4299986;
        let cities = same_latitude(lat);
        let mut names: Vec<_> = cities.iter().map(|ref c| &c.name).collect();
        names.sort();
        assert_eq!(
            names,
            vec!["Adapazari", "Agdam", "Alexandroupoli", "Ali Bayramli", "Allentown", "Altoona", "Amasya", "Andijon", "Anxi", "Aomori", "Arcata", "Artashat", "Ashtarak", "Aveiro", "Baku", "Baotou", "Beaver Falls", "Berat", "Bilecik", "Bloomington", "Bolu", "Boulder", "Brindisi", "Burlington", "Bursa", "Canakkale", "Cankiri", "Canton", "Castello", "Changping", "Chosan", "Coimbra", "Columbus", "Corovode", "Corum", "Covilha", "Craig", "Dandong", "Datong", "Dunhuang", "Elko", "Erseke", "Eureka", "Fargona", "Fengzhen", "Fier", "Fort Collins", "Gadabay", "Ganca", "Gavarr", "Giresun", "Gjirokaster", "Goranboy", "Goycay", "Gramsh", "Grand Island", "Greeley", "Guadalajara", "Guarda", "Guliston", "Gumushane", "Gyumri", "Hachinohe", "Hanggin Houqi", "Harrisburg", "Hirosaki", "Hohhot", "Ijevan", "Izmit", "Jinxi", "Jizzax", "Johnstown", "Kars", "Katerini", "Kearney", "Khujand", "Kimchaek", "Kimhyonggwon", "Kirksville", "Kokomo", "Konibodom", "Korce", "Lafayette", "Lancaster", "Lecce", "Lima", "Lincoln", "Madrid", "Mansfield", "Marion", "McCook", "Muncie", "Naples", "Navoi", "New York", "Newark", "Olbia", "Olmaliq", "Osh", "Paterson", "Peoria", "Permet", "Philadelphia", "Pittsburgh", "Pogradec", "Polygyros", "Potenza", "Provo", "Qinhuangdao", "Qoqon", "Quincy", "Redding", "Sakarya", "Salerno", "Salt Lake City", "Sassari", "Sinuiju", "State College", "Sumqayt", "Taedong", "Taranto", "Tepelene", "Thessaloniki", "Tokat", "Trenton", "Turkmenbasy", "Urbana", "Vanadzor", "Vernal", "Viseu", "Vlore", "Wheeling", "Xuanhua", "Yerevan", "Yevlax", "Yingkow", "York", "Zanesville", "Zhangjiakou"]
        );
    }

    #[test]
    fn it_filters_to_ten_by_population() {
        let lat = 40.4299986;
        let cities = same_latitude(lat);

        let cities = top_10_by_population(cities);

        let names: Vec<_> = cities.iter().map(|ref c| &c.name).collect();

        assert_eq!(
            names,
            vec!["New York", "Philadelphia", "Madrid", "Baku", "Naples", "Pittsburgh", "Datong", "Bursa", "Jinxi", "Hohhot"]
        );
    }

    #[test]
    fn it_sorts_easterly() {
        let lat = 40.4299986;
        let long = -79.99998539;
        let cities = same_latitude(lat);
        let cities = top_10_by_population(cities);

        let cities = sort_easterly(cities, long);

        let names: Vec<_> = cities.iter().map(|ref c| &c.name).collect();

        assert_eq!(
            names,
            vec!["Philadelphia", "New York", "Madrid", "Naples", "Bursa", "Baku", "Hohhot", "Datong", "Jinxi", "Pittsburgh"]
        );
    }

    #[test]
    fn it_creates_latitude_text() {
        let lat = 40.4299986;
        let long = -79.99998539;

        let latitude_text = latitude_text(lat, long);

        assert_eq!(
            latitude_text,
            "If you fly along this latitude in an easterly direction, you will look down on Philadelphia, New York, Madrid, Naples, Bursa, Baku, Hohhot, Datong, Jinxi, Pittsburgh."
        );
    }

    #[test]
    fn it_finds_longitude_on_other_side_of_the_world() {
        let long = -79.99998539;
        assert_approx_eq!(
            opposite_longitude(long),
            100.00001461
        );

        let long = long * -1.0;
        assert_approx_eq!(
            opposite_longitude(long),
            -100.00001461
        );
    }

    #[test]
    fn it_finds_cities_with_same_longitude() {
        let long = -79.99998539;
        let cities = same_longitude(long);
        let mut names: Vec<_> = cities.iter().map(|ref c| &c.name).collect();
        names.sort();
        assert_eq!(
            names,
            vec!["Alor Setar", "Ang Thong", "Babahoyo", "Balboa", "Ban Houayxay", "Barrie", "Beaver Falls", "Blacksburg", "Bukittinggi", "Butterworth", "Chainat", "Charleston", "Chiang Rai", "Chiclayo", "Chitre", "Chone", "Chulucanas", "Cienfuegos", "Clarksburg", "Cobalt", "Colon", "Coral Gables", "Coral Springs", "Dali", "Erie", "Esmeraldas", "Ferrenafe", "Florence", "Fort Lauderdale", "Fort Pierce", "George Town", "Greensboro", "Guayaquil", "Hamilton", "Hat Yai", "Homestead", "Hua Hin", "Kamphaeng Phet", "Kanchanaburi", "Kangar", "Las Tablas", "Lijiang", "Macara", "Machala", "Miami", "Miami Beach", "Milagro", "Morgantown", "Moron", "Motupe", "Muisne", "Nakhon Pathom", "Nakhon Sawan", "Nakhon Si Thammarat", "New Liskeard", "Nonthaburi", "Olmos", "Orangeville", "Pacasmayo", "Padang", "Padangpanjang", "Panama City", "Parry Sound", "Penonome", "Phatthalung", "Phayao", "Phetchaburi", "Phichit", "Phitsanulok", "Phrae", "Pimentel", "Pinas", "Pittsburgh", "Placetas", "Portoviejo", "Prachuap Khiri Khan", "Ratchaburi", "Roanoke", "Sagua la Grande", "Salisbury", "Samut Sakhon", "Samut Songkhram", "Santa Clara", "Satun", "Sing Buri", "Sukhothai", "Sumter", "Sungai Petani", "Supham Buri", "Thung Song", "Trang", "Tumbes", "Tura", "Uthai Thani", "Uttaradit", "Vero Beach", "West Palm Beach", "White Sulphur Springs", "Winston-Salem", "Zhangye"]
        );
    }

    #[test]
    fn it_filters_longitude_cities_to_ten_by_population() {
        let long = -79.99998539;
        let cities = same_longitude(long);

        let cities = top_10_by_population(cities);

        let names: Vec<_> = cities.iter().map(|ref c| &c.name).collect();

        assert_eq!(
            names,
            vec!["Miami", "Guayaquil", "George Town", "Pittsburgh", "Fort Lauderdale", "Padang", "Panama City", "West Palm Beach", "Hamilton", "Chiclayo"]
        );
    }

    #[test]
    fn it_sorts_northerly() {
        let lat = 40.4299986;
        let long = -79.99998539;
        let cities = same_longitude(long);
        let cities = top_10_by_population(cities);

        let cities = sort_northerly(cities, lat, long);

        let names: Vec<_> = cities.iter().map(|ref c| &c.name).collect();

        assert_eq!(
            names,
            vec!["Hamilton", "North Pole", "George Town", "Padang", "South Pole", "Chiclayo", "Guayaquil", "Panama City", "Miami", "Fort Lauderdale", "West Palm Beach", "Pittsburgh"]
        );
    }
}
