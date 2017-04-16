extern crate csv;
extern crate rustc_serialize;

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

fn decimal_to_degrees_minutes(coord: f64) -> (f64, f64) {
    (
        coord.abs().floor(),
        ((coord.abs() * 60.0) % 60.0).floor()
    )
}

fn main() {
    let mut csv = csv::Reader::from_file("./simplemaps-worldcities-basic.csv")
        .unwrap();

    for record in csv.decode() {
        let record: Record = record.unwrap();

        let (lat_deg, lat_min) = decimal_to_degrees_minutes(record.latitude);
        let (long_deg, long_min) = decimal_to_degrees_minutes(record.longitude);

        println!(
            "{},{},{},{},{},{},{},{}",
            record.city,
            lat_deg,
            lat_min,
            if record.latitude < 0.0 { 'S' } else { 'N' },
            long_deg,
            long_min,
            if record.longitude < 0.0 { 'W' } else { 'E' },
            record.population
        );
    }
}
