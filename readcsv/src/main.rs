use csv::Reader;

fn main() -> Result<(), csv::Error> {
    let mut reader = Reader::from_path("country.csv").unwrap();

    for result in reader.records() {
        let record = result.unwrap();
        println!("{:?}", record);
    }
    Ok(())
}
