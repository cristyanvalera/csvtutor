use std::{error::Error, process, io};

// type Record = (String, String, Option<u64>, f64, f64);

fn run() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        let record = result?;
        
        let city = &record[0];
        let state = &record[1];
        let pop: Option<u64> = record[2].parse().ok();
        let latitude: f64 = record[3].parse()?;
        let longitude: f64 = record[4].parse()?;

        println!(
            "city: {:?}, state: {:?}, \
            pop: {:?}, latitude: {:?}, longitude: {:?}",
            city, state, pop, latitude, longitude);
    }

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
        process::exit(1);
    }
}
