use std::{error::Error, process, io};

fn run() -> Result<(), Box<dyn Error>> {
    // let file_path = get_first_arg()?;
    // let mut rdr = csv::Reader::from_path(file_path)?;

    // for result in rdr.records() {
    //     let record = result?;
    //     println!("{:?}", record);
    // }

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b';')
        .double_quote(false)
        .escape(Some(b'\\'))
        .flexible(true)
        .comment(Some(b'#'))
        .from_reader(io::stdin());

    for result in rdr.records() {
        let record = result?;

        println!("{:?}", record);
    }

    Ok(())
}

// fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
//     match env::args_os().nth(1) {
//         None => Err(From::from("Expected 1 argument, but got none.")),
//         Some(file_path) => Ok(file_path),
//     }
// }

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
        process::exit(1);
    }
}
