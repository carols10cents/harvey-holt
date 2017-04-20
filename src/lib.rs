#[macro_use]
extern crate lazy_static;
extern crate csv;
extern crate rustc_serialize;

use std::convert::From;

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

#[derive(PartialEq, Clone, Debug)]
struct Coord {
    deg: u8,
    min: u8,
    dir: char,
}

#[derive(PartialEq, Clone, Debug)]
struct City {
    name: String,
    latitude: Coord,
    longitude: Coord,
    population: f64,
}

impl From<Record> for City {
    fn from(record: Record) -> City {
        City {
            name: record.city,
            latitude: Coord {
                deg: record.lat_deg,
                min: record.lat_min,
                dir: record.lat_dir,
            },
            longitude: Coord {
                deg: record.long_deg,
                min: record.long_min,
                dir: record.long_dir,
            },
            population: record.population,
        }
    }
}

struct Cities {
    same_latitude: Vec<City>,
    same_longitude: Vec<City>,
}

lazy_static! {
    static ref DATA: Vec<City> = csv::Reader::from_file("./data.csv")
        .unwrap()
        .decode::<Record>()
        .map(Result::unwrap)
        .map(Record::into)
        .collect();
}

fn same_latitude(lat: Coord) -> Vec<City> {
    DATA
        .iter()
        .cloned()
        .filter(|city| {
            city.latitude.deg == lat.deg && city.latitude.dir == lat.dir
        })
        .collect()
}

fn top_10_by_population(mut cities: Vec<City>) -> Vec<City> {
    cities.sort_by(|a, b| b.population.partial_cmp(&a.population).unwrap());
    cities.into_iter().take(10).collect()
}

fn sort_easterly(cities: Vec<City>, start_long: Coord) -> Vec<City> {
    if start_long.dir == 'W' {
        let mut start_to_prime_meridian: Vec<_> = cities.iter().filter(|c| {
            c.longitude.dir == 'W' &&
            (c.longitude.deg < start_long.deg || (
                c.longitude.deg == start_long.deg &&
                c.longitude.min < start_long.min
            ))
        }).cloned().collect();
        start_to_prime_meridian.sort_by(|a, b| {
            b.longitude.deg.cmp(&a.longitude.deg)
        });

        let mut eastern_hemisphere: Vec<_> = cities.iter().filter(|c| {
            c.longitude.dir == 'E'
        }).cloned().collect();
        eastern_hemisphere.sort_by(|a, b| {
            a.longitude.deg.cmp(&b.longitude.deg)
        });

        let mut date_line_to_start: Vec<_> = cities.iter().filter(|c| {
            c.longitude.dir == 'W' &&
            (c.longitude.deg > start_long.deg || (
                c.longitude.deg == start_long.deg &&
                c.longitude.min >= start_long.min
            ))
        }).cloned().collect();
        date_line_to_start.sort_by(|a, b| {
            b.longitude.deg.cmp(&a.longitude.deg)
        });

        let mut result = Vec::with_capacity(cities.len());
        result.extend(start_to_prime_meridian);
        result.extend(eastern_hemisphere);
        result.extend(date_line_to_start);
        result
    } else {
        unimplemented!();
        // let start_to_date_line =
        // let western_hemisphere =
        // let prime_meridian_to_start =
        //
        // let result = Vec::with_capacity(cities.len());
        // result.push_all(start_to_prime_meridian);
        // result.push_all(eastern_hemisphere);
        // result.push_all(date_line_to_start);
        // result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_finds_cities_with_same_latitude() {
        let lat = Coord { deg: 40, min: 25, dir: 'N' };
        let cities = same_latitude(lat);
        let mut names: Vec<_> = cities.iter().map(|ref c| &c.name).collect();
        names.sort();
        assert_eq!(
            names,
            vec!["Adapazari", "Agdam", "Alexandroupoli", "Allentown", "Altoona", "Amasya", "Andijon", "Anxi", "Aomori", "Arcata", "Ashtarak", "Aveiro", "Baku", "Baotou", "Beaver Falls", "Berat", "Bilecik", "Bloomington", "Bolu", "Boulder", "Brindisi", "Burlington", "Bursa", "Canakkale", "Cankiri", "Canton", "Changping", "Chengde", "Chosan", "Coimbra", "Corovode", "Corum", "Covilha", "Craig", "Dandong", "Datong", "Dunhuang", "Elko", "Erseke", "Eureka", "Fargona", "Fengzhen", "Fier", "Fort Collins", "Gadabay", "Galesburg", "Ganca", "Gavarr", "Giresun", "Gjirokaster", "Goranboy", "Goycay", "Gramsh", "Grand Island", "Greeley", "Guadalajara", "Guarda", "Guliston", "Gumushane", "Gyumri", "Hachinohe", "Hanggin Houqi", "Harrisburg", "Hirosaki", "Hohhot", "Ijevan", "Izmit", "Jalal Abad", "Jinxi", "Jizzax", "Johnstown", "Kanggye", "Kars", "Katerini", "Kavala", "Kearney", "Khujand", "Kilchu", "Kimchaek", "Kimhyonggwon", "Kirksville", "Kokomo", "Konibodom", "Korce", "Lafayette", "Lancaster", "Lecce", "Lima", "Lincoln", "Lushnje", "Madrid", "Mansfield", "Marion", "McCook", "Muncie", "Naples", "Navoi", "New York", "Newark", "Olbia", "Olmaliq", "Osh", "Paterson", "Peoria", "Permet", "Pittsburgh", "Pogradec", "Polygyros", "Potenza", "Provo", "Qabala", "Qoqon", "Redding", "Sakarya", "Salamanca", "Salerno", "Salt Lake City", "Sassari", "Sinuiju", "State College", "Sumqayt", "Taedong", "Taranto", "Tekirdag", "Tepelene", "Thessaloniki", "Tokat", "Tovuz", "Trabzon", "Trenton", "Turkmenbasy", "Urbana", "Vanadzor", "Vernal", "Viseu", "Vlore", "Wheeling", "Winnemucca", "Xuanhua", "Yerevan", "Yevlax", "Yingkow", "Zhangjiakou"]
        );
    }

    #[test]
    fn it_filters_to_ten_by_population() {
        let lat = Coord { deg: 40, min: 25, dir: 'N' };
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
        let lat = Coord { deg: 40, min: 25, dir: 'N' };
        let long = Coord { deg: 79, min: 59, dir: 'W' };
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
