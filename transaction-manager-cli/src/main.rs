use clap::Parser;
use std::fs::File;
use std::error::Error;
use csv::ReaderBuilder;
use transaction_manager_lib::transactions::{Transaction};

mod cli;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = cli::Cli::parse();

    println!("cli: {:?}", cli.input);

    let file = File::open(cli.input)?;

    // Configure the CSV reader to trim headers and fields of whitespace
    let mut rdr = ReaderBuilder::new()
        .trim(csv::Trim::All) // Trims all fields including headers
        .from_reader(file);

    for result in rdr.deserialize::<Transaction>() {
        println!("result: {result:?}");

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
