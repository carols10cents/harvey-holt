#[macro_use]
extern crate lazy_static;
extern crate csv;
extern crate rustc_serialize;

#[derive(RustcDecodable)]
struct Record {
    city: String,
    lat_deg: u8,
    lat_min: u8,
    lat_dir: char,
    long_deg: u8,
    long_min: u8,
    long_dir: char,
    population: f64,
}

#[derive(PartialEq)]
struct Coord {
    deg: u8,
    min: u8,
    dir: char,
}

#[derive(PartialEq)]
struct City {
    name: String,
    latitude: Coord,
    longitude: Coord,
    population: f64,
}

struct Cities {
    same_latitude: Vec<City>,
    same_longitude: Vec<City>,
}

lazy_static! {
    static ref DATA: Vec<Record> = csv::Reader::from_file("./data.csv")
        .unwrap()
        .decode()
        .map(Result::unwrap)
        .collect();
}

fn same_latitude(lat: Coord) -> Vec<City> {
    DATA
        .iter()
        .filter(|r| {
            r.lat_deg == lat.deg && r.lat_min == lat.min && r.lat_dir == lat.dir
        })
        .map(|r| {
            City {
                name: r.city.clone(),
                latitude: Coord {
                    deg: r.lat_deg,
                    min: r.lat_min,
                    dir: r.lat_dir,
                },
                longitude: Coord {
                    deg: r.long_deg,
                    min: r.long_min,
                    dir: r.long_dir,
                },
                population: r.population,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_finds_cities_with_same_latitude() {
        let lat = Coord { deg: 40, min: 25, dir: 'N'};
        let cities = same_latitude(lat);
        let mut cities: Vec<_> = cities.iter().map(|ref c| &c.name).collect();
        cities.sort();
        assert_eq!(
            cities,
            vec!["Greeley", "Lafayette", "Pittsburgh"]
        );
    }
}
