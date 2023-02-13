use std::{io, process, error::Error};

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());

    for result in rdr.records() {
       let record = result?;
       println!("{:?}", record);
    }

    Ok(())
}
