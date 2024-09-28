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
    // TODO: Made this Arc, thinking we may want to paralellize this in the future
    //   and put it behind an Arc<RwLock>
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
        debug!("{w:?}");

        Ok(())
    }

    fn handle_deposit(&mut self, d: &Deposit) -> Result<(), TransactionManagerError> {
        debug!("{d:?}");

        Ok(())
    }

    fn handle_chargeback(&mut self, c: &Chargeback) -> Result<(), TransactionManagerError> {
        debug!("{c:?}");

        Ok(())
    }

    fn handle_dispute(&mut self, d: &Dispute) -> Result<(), TransactionManagerError> {
        debug!("{d:?}");

        Ok(())
    }

    fn handle_resolve(&mut self, r: &Resolve) -> Result<(), TransactionManagerError> {
        debug!("{r:?}");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use crate::balance::{ClientBalance, ClientBalanceRegistry};
    use crate::transactions::{Transaction, Deposit, Withdrawal, Chargeback, Dispute, Resolve};

    // TODO: Consider a few corner cases here
    // * rejecting transaction IDs we've already seen?
    
    #[test]
    fn test_rejecting_duplicate_transaction_ids() {

    }

    #[test]
    fn test_simple_deposit_withdrawal() {
        let mut tm = TransactionManager::new();

        let deposit = Transaction::Deposit(Deposit::new(1, 1, 32.0));
        let withdrawal = Transaction::Withdrawal(Withdrawal::new(1, 2, 20.0));

        tm.attempt_insertion(&deposit).unwrap();
        tm.attempt_insertion(&withdrawal).unwrap();

        let mut internal = HashMap::new();
        let client_1_balance = ClientBalance::new(20.0, 0.0, 0.0, 0.0);
        internal.insert(1, client_1_balance);
        let expected_balances = Arc::new(ClientBalanceRegistry::load_registry(internal));

        let actual_balance = tm.retrieve_client_balances();

        assert_eq!(actual_balance, expected_balances);
    }
}

