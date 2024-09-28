use clap::Parser;
use std::fs::File;
use std::error::Error;
use csv::ReaderBuilder;
use transaction_manager_lib::transactions::{Transaction};
use transaction_manager_lib::transaction_manager::{TransactionManager};
use log::*;

mod cli;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    
    let cli = cli::Cli::parse();
    trace!("cli: {:?}", cli.input);
    let file = File::open(cli.input)?;
    // Configuring to make sure we trim all whitespace from headers and fields
    let mut rdr = ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_reader(file);

    let mut transaction_manager = TransactionManager::new();

    for result in rdr.deserialize::<Transaction>() {
        debug!("result: {result:?}");

        let Ok(transaction) = result else {
            // If we get a malformed transaction, we just
            // continue to read
            continue;
        };

        debug!("transaction: {transaction:?}");

        if let Err(e) = transaction_manager.record_transaction(&transaction) {
            warn!("Transaction failed to be inserted: transaction: {transaction:?} err: {e:?}");
        }
    }

    let client_balance_registry = transaction_manager.retrieve_client_balances();

    println!("{}", client_balance_registry.to_csv());

    Ok(())
}
