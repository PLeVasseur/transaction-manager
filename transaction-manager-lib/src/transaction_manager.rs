use std::fmt;
use log::*;
use std::sync::Arc;
use crate::transactions::{Transaction, Withdrawal, Deposit, Chargeback, Resolve, Dispute};
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

pub struct TransactionManager {
    balances: Arc<ClientBalanceRegistry>
}

impl TransactionManager {
    pub fn new() -> Self {
        Self {
            balances: Arc::new(ClientBalanceRegistry::new())
        }
    }

    pub fn attempt_insertion(&mut self, t: &Transaction) -> Result<(), TransactionManagerError> {
        debug!("Transaction: {t:?}");

        match t {
            Transaction::Withdrawal(w) => self.handle_withdrawal(w),
            Transaction::Deposit(d) => self.handle_deposit(d),
            Transaction::Chargeback(c) => self.handle_chargeback(c),
            Transaction::Resolve(r) => self.handle_resolve(r),
            Transaction::Dispute(d) => self.handle_dispute(d),
        }
    }

    pub fn retrieve_client_balances(&self) -> Arc<ClientBalanceRegistry> {
        self.balances.clone()
    }

    fn handle_withdrawal(&mut self, w: &Withdrawal) -> Result<(), TransactionManagerError> {
        Ok(())
    }

    fn handle_deposit(&mut self, d: &Deposit) -> Result<(), TransactionManagerError> {
        Ok(())
    }

    fn handle_chargeback(&mut self, c: &Chargeback) -> Result<(), TransactionManagerError> {
        Ok(())
    }

    fn handle_dispute(&mut self, d: &Dispute) -> Result<(), TransactionManagerError> {
        Ok(())
    }

    fn handle_resolve(&mut self, r: &Resolve) -> Result<(), TransactionManagerError> {
        Ok(())
    }
}

