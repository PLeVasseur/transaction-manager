use std::fmt;
use crate::transactions::Transaction;
use crate::balance::ClientBalanceRegistry;

#[derive(Debug)]
pub enum TransactionManagerError {
    // TODO: Consider if we want something more sophisticated
    InvalidTransaction(String)
}

impl fmt::Display for TransactionManagerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TransactionManagerError::InvalidTransaction(reason) => write!(f, "Invalid transaction: {reason}"),
        }
    }
}

impl std::error::Error for TransactionManagerError {}

pub struct TransactionManager {}

impl TransactionManager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn attempt_insertion(&mut self, t: &Transaction) -> Result<(), TransactionManagerError> {
        println!("Transaction: {t:?}");

        Ok(())
    }

    pub fn retrieve_client_balances(&self) -> ClientBalanceRegistry {
        ClientBalanceRegistry::new()
    }
}
