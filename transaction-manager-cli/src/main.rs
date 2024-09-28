use clap::Parser;
use std::fs::File;
use std::error::Error;
use csv::ReaderBuilder;
use transaction_manager_lib::transactions::{Transaction};
use transaction_manager_lib::transaction_manager::{TransactionManager};

mod cli;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = cli::Cli::parse();
    println!("cli: {:?}", cli.input);
    let file = File::open(cli.input)?;
    // Configure the CSV reader to trim headers and fields of whitespace
    let mut rdr = ReaderBuilder::new()
        .trim(csv::Trim::All) // Trims all fields including headers
        .from_reader(file);

    let mut transaction_manager = TransactionManager::new();

    for result in rdr.deserialize::<Transaction>() {
        println!("result: {result:?}");

        let Ok(transaction) = result else {
            // If we get a malformed transaction, we just
            // continue to read
            continue;
        };

        println!("transaction: {transaction:?}");

        if let Err(e) = transaction_manager.attempt_insertion(&transaction) {
            eprintln!("Transaction failed to be inserted: transaction: {transaction:?} err: {e:?}");
        }
    }

    let client_balance_registry = transaction_manager.retrieve_client_balances();

    println!("Hello, world!");

    println!("{}", client_balance_registry.to_csv());

    Ok(())
}
