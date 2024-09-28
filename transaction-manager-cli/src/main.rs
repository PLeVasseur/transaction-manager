use clap::Parser;
use std::fs::File;
use std::error::Error;
use csv::Reader;

mod cli;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = cli::Cli::parse();

    println!("cli: {:?}", cli.input);

    let file = File::open(cli.input)?;
    let mut rdr = Reader::from_reader(file);

    for result in rdr.records() {
        let Ok(transaction) = result else {
            // If we get a malformed transaction, we just
            // continue to read
            continue;
        };

        println!("transaction: {transaction:?}");
    }

    println!("Hello, world!");

    Ok(())
}
