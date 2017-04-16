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

lazy_static! {
    static ref DATA: Vec<Record> = csv::Reader::from_file("./data.csv")
        .unwrap()
        .decode()
        .map(Result::unwrap)
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(DATA.len(), 7320);
    }
}