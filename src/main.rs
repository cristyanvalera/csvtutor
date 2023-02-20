use std::{error::Error, process, io, collections::HashMap};

type Record = HashMap<String, String>;

fn run() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());

    for result in rdr.deserialize() {
        let record: Record = result?;
        
        println!("{:?}", record);
    }

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
        process::exit(1);
    }
}
