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

#[derive(PartialEq, Clone)]
struct Coord {
    deg: u8,
    min: u8,
    dir: char,
}

#[derive(PartialEq, Clone)]
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
}
