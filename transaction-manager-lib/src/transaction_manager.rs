use std::clone::Clone;
use std::fmt;
use log::*;
use std::sync::{Arc, RwLock};
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
    // TODO: Made this an Arc<RwLock>, thinking we may have multiple
    // concurrent requests to record transactions 
    // (although, there's probably a better way to do that still!)
    balances: Arc<RwLock<ClientBalanceRegistry>>
}

impl TransactionManager {
    pub fn new() -> Self {
        Self {
            balances: Arc::new(RwLock::new(ClientBalanceRegistry::new()))
        }
    }

    pub fn record_transaction(&mut self, t: &Transaction) -> Result<(), TransactionManagerError> {
        debug!("Transaction: {t:?}");

        match t {
            Transaction::Withdrawal(w) => self.handle_withdrawal(w),
            Transaction::Deposit(d) => self.handle_deposit(d),
            Transaction::Chargeback(c) => self.handle_chargeback(c),
            Transaction::Resolve(r) => self.handle_resolve(r),
            Transaction::Dispute(d) => self.handle_dispute(d),
        }
    }

    pub fn retrieve_client_balances(&self) -> ClientBalanceRegistry {
        let balance = self.balances.read().unwrap();

        (*balance).clone()
    }

    // Documenting an assumption here:
    //   * given the lack of direction to refuse withdrawals which would make the
    //     total negative, I've chosen to allow account balances to go negative
    fn handle_withdrawal(&mut self, w: &Withdrawal) -> Result<(), TransactionManagerError> {
        debug!("{w:?}");

        let mut registry = self.balances.write().unwrap();

        let client_account = registry.client_balances.entry(w.client).or_default();
        trace!("client_account, prior: {client_account:?}");

        client_account.total -= w.amount;
        client_account.available -= w.amount;

        trace!("client_account, after: {client_account:?}");

        Ok(())
    }

    fn handle_deposit(&mut self, d: &Deposit) -> Result<(), TransactionManagerError> {
        debug!("{d:?}");

        let mut registry = self.balances.write().unwrap();

        let client_account = registry.client_balances.entry(d.client).or_default();
        trace!("client_account, prior: {client_account:?}");

        client_account.total += d.amount;
        client_account.available += d.amount;

        trace!("client_account, after: {client_account:?}");

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
    use std::sync::Once;
    use std::collections::HashMap;
    use crate::balance::{ClientBalance, ClientBalanceRegistry};
    use crate::transactions::{Transaction, Deposit, Withdrawal, Chargeback, Dispute, Resolve};

    // TODO: Consider a few corner cases here
    // * rejecting transaction IDs we've already seen?
    // * rejecting if amount is negative?

    static INIT: Once = Once::new();
    
    fn test_setup() {
        INIT.call_once(|| env_logger::init());
    }

    #[test]
    fn test_rejecting_duplicate_transaction_ids() {

    }

    #[test]
    fn test_negative_amount_deposit() {
        
    }

    #[test]
    fn test_simple_deposit_withdrawal() {
        test_setup();

        let mut tm = TransactionManager::new();

        let deposit = Transaction::Deposit(Deposit::new(1, 1, 32.0));
        let withdrawal = Transaction::Withdrawal(Withdrawal::new(1, 2, 20.0));

        tm.record_transaction(&deposit).unwrap();
        tm.record_transaction(&withdrawal).unwrap();

        let mut internal = HashMap::new();
        let client_1_balance = ClientBalance::new(12.0, 0.0, 12.0, false);
        internal.insert(1, client_1_balance);
        let expected_balances = ClientBalanceRegistry::load_registry(internal);

        let actual_balance = tm.retrieve_client_balances();

        assert_eq!(actual_balance, expected_balances);
    }

    // TODO: Using test-case would probably simplify this by letting us have various
    // setups which differ only in the expected input and output
    #[test]
    fn test_multiple_clients_deposit_withdrawal() {
        test_setup();

        let mut tm = TransactionManager::new();


        let transactions = vec![
            Transaction::Withdrawal(Withdrawal::new(2, 4, 200.0)),
            Transaction::Deposit(Deposit::new(1, 1, 32.0)),
            Transaction::Withdrawal(Withdrawal::new(1, 2, 20.0)),
            Transaction::Deposit(Deposit::new(2, 3, 2.0)),
        ];

        for transaction in &transactions {
            tm.record_transaction(transaction).unwrap();
        }
        let client_1_balance = ClientBalance::new(12.0, 0.0, 12.0, false);
        let client_2_balance = ClientBalance::new(-198.0, 0.0, -198.0, false);
        let internal = HashMap::from([
          (1, client_1_balance),
          (2, client_2_balance),
        ]);
        let expected_balances = ClientBalanceRegistry::load_registry(internal);

        let actual_balance = tm.retrieve_client_balances();

        assert_eq!(actual_balance, expected_balances);
    }

}

