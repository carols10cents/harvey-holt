#![feature(ordering_chaining)]

#[macro_use]
extern crate lazy_static;
extern crate csv;
extern crate rustc_serialize;

use std::convert::From;
use std::cmp::Ordering;

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

fn top_10_by_population(mut cities: Vec<City>) -> Vec<City> {
    cities.sort_by(|a, b| b.population.partial_cmp(&a.population).unwrap());
    cities.into_iter().take(10).collect()
}

fn sort_easterly(cities: Vec<City>, start_long: f64) -> Vec<City> {
    cities
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
            vec!["New York", "Madrid", "Baku", "Naples", "Pittsburgh", "Datong", "Bursa", "Jinxi", "Hohhot", "Baotou"]
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
            vec!["New York", "Madrid", "Naples", "Bursa", "Baku", "Baotou", "Hohhot", "Datong", "Jinxi", "Pittsburgh"]
        );
    }
}
