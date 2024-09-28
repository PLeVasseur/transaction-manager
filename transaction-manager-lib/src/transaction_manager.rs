use crate::balance::ClientBalanceRegistry;
use crate::history::TransactionHistory;
use crate::transactions::{Chargeback, Deposit, Dispute, Resolve, Transaction, Withdrawal};
use log::*;
use std::clone::Clone;
use std::fmt;
use std::sync::{Arc, RwLock};

#[derive(Debug, PartialEq)]
pub enum TransactionManagerError {
    // TODO: Consider if we want something more sophisticated
    InvalidTransaction(String),
    InsufficientFunds(f64),
    AccountLocked,
    DuplicateTransactionId(u32),
    DisputedTransactionDoesNotExist(u32),
    NegativeAmountNotAllowed,
}

impl fmt::Display for TransactionManagerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TransactionManagerError::InvalidTransaction(reason) => {
                write!(f, "InvalidTransaction: {reason}")
            }
            TransactionManagerError::InsufficientFunds(insufficient_amount) => {
                write!(f, "InsufficientFunds({insufficient_amount}")
            }
            TransactionManagerError::AccountLocked => write!(f, "AccountLocked"),
            TransactionManagerError::DuplicateTransactionId(duped_id) => {
                write!(f, "DuplicateTransactionId({duped_id}")
            }
            TransactionManagerError::DisputedTransactionDoesNotExist(tx) => {
                write!(f, "DisputedTransactionDoesNotExist({tx}")
            }
            TransactionManagerError::NegativeAmountNotAllowed => {
                write!(f, "NegativeAmountNotAllowed")
            }
        }
    }
}

impl std::error::Error for TransactionManagerError {}

pub struct TransactionManager {
    // TODO: Made this an Arc<RwLock>, thinking we may have multiple
    // concurrent requests to record transactions
    // (although, there's probably a better way to do that still!)
    balances: Arc<RwLock<ClientBalanceRegistry>>,
    history: TransactionHistory,
}

impl TransactionManager {
    pub fn new() -> Self {
        Self {
            balances: Arc::new(RwLock::new(ClientBalanceRegistry::new())),
            history: TransactionHistory::new(),
        }
    }

    pub fn record_transaction(&mut self, t: &Transaction) -> Result<(), TransactionManagerError> {
        debug!("Transaction: {t:?}");

        // TODO: I'd like to reject duplicate transactions here, but because of how I currently
        // have the id setup as a part of each individual transaction type, I cannot
        // So I'll have to have some duplicate code in each of the below methods unfortunately

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

    fn duped_transaction(&self, tx: &u32) -> Result<(), TransactionManagerError> {
        if self.history.get(tx).is_some() {
            return Err(TransactionManagerError::DuplicateTransactionId(*tx));
        }

        Ok(())
    }

    fn reject_negative_amount(&self, amount: &f64) -> Result<(), TransactionManagerError> {
        if *amount < 0.0 {
            return Err(TransactionManagerError::NegativeAmountNotAllowed);
        }

        Ok(())
    }

    fn handle_withdrawal(&mut self, w: &Withdrawal) -> Result<(), TransactionManagerError> {
        debug!("{w:?}");

        self.duped_transaction(&w.tx)?;
        self.reject_negative_amount(&w.amount)?;

        let mut registry = self.balances.write().unwrap();

        let client_account = registry.client_balances.entry(w.client).or_default();
        trace!("client_account, prior: {client_account:?}");

        if client_account.locked {
            return Err(TransactionManagerError::AccountLocked);
        }

        let remaining_amount = client_account.available - w.amount;

        if remaining_amount < 0.0 {
            return Err(TransactionManagerError::InsufficientFunds(
                remaining_amount * -1.0,
            ));
        }

        client_account.total -= w.amount;
        client_account.available -= w.amount;

        trace!("client_account, after: {client_account:?}");

        self.history
            .insert(w.tx, Transaction::Withdrawal(w.clone()));

        trace!("history: {:?}", self.history);

        Ok(())
    }

    fn handle_deposit(&mut self, d: &Deposit) -> Result<(), TransactionManagerError> {
        debug!("{d:?}");

        self.duped_transaction(&d.tx)?;
        self.reject_negative_amount(&d.amount)?;

        let mut registry = self.balances.write().unwrap();

        let client_account = registry.client_balances.entry(d.client).or_default();
        trace!("client_account, prior: {client_account:?}");

        if client_account.locked {
            return Err(TransactionManagerError::AccountLocked);
        }

        client_account.total += d.amount;
        client_account.available += d.amount;

        trace!("client_account, after: {client_account:?}");

        self.history.insert(d.tx, Transaction::Deposit(d.clone()));

        trace!("history: {:?}", self.history);

        Ok(())
    }

    // TODO: It seems to me that disputes really only apply to deposits, right?
    // When I read the text it seems to indicate that, e.g. "This means
    // that the clients available funds should decrease by the amount disputed"
    // => Performing this operation on a transaction that's a withdrawal would then
    // seem to mean that we're decreasing the available funds, or would we in that
    // case be increasing the available funds? Hmm. Let's start by considering
    // only deposits and revisit
    fn handle_dispute(&mut self, d: &Dispute) -> Result<(), TransactionManagerError> {
        debug!("{d:?}");

        let mut registry = self.balances.write().unwrap();

        let client_account = registry.client_balances.entry(d.client).or_default();
        trace!("client_account, prior: {client_account:?}");

        if client_account.locked {
            return Err(TransactionManagerError::AccountLocked);
        }

        let disputed_transaction = self.history.get(&d.tx);
        trace!("disputed_transaction: {disputed_transaction:?}");

        // Assuming that disputes, resolves, and chargebacks only apply to deposits,
        // which seems to make sense
        let Some(Transaction::Deposit(dep)) = disputed_transaction else {
            return Err(TransactionManagerError::DisputedTransactionDoesNotExist(
                d.tx,
            ));
        };

        // TODO: Is it possible for this to go negative? Should check
        client_account.available -= dep.amount;
        client_account.held += dep.amount;

        client_account.disputed_transactions.insert(d.tx);

        trace!("client_account, after: {client_account:?}");

        Ok(())
    }

    // TODO: It would appear that there's not a description of what operations should
    // be allowed if an account is locked / frozen (as far as I can tell).
    // => ASSUMPTION: Locked accounts can have not operations performed on them,
    //                perhaps they need some sort of manual intervention
    fn handle_chargeback(&mut self, c: &Chargeback) -> Result<(), TransactionManagerError> {
        debug!("{c:?}");

        // TODO: Need to allow like... a single dispute at once?

        let mut registry = self.balances.write().unwrap();

        let client_account = registry.client_balances.entry(c.client).or_default();
        trace!("client_account, prior: {client_account:?}");

        if client_account.locked {
            return Err(TransactionManagerError::AccountLocked);
        }

        let disputed_transaction = self.history.get(&c.tx);
        trace!("disputed_transaction: {disputed_transaction:?}");

        // Assuming that disputes, resolves, and chargebacks only apply to deposits,
        // which seems to make sense
        let Some(Transaction::Deposit(dep)) = disputed_transaction else {
            return Err(TransactionManagerError::DisputedTransactionDoesNotExist(
                c.tx,
            ));
        };

        // TODO: Is it possible for this to go negative? Should check
        client_account.total -= dep.amount;
        client_account.held -= dep.amount;

        let _ = client_account.disputed_transactions.remove(&c.tx);

        client_account.locked = true;

        trace!("client_account, after: {client_account:?}");

        Ok(())
    }

    fn handle_resolve(&mut self, r: &Resolve) -> Result<(), TransactionManagerError> {
        debug!("{r:?}");

        let mut registry = self.balances.write().unwrap();

        let client_account = registry.client_balances.entry(r.client).or_default();
        trace!("client_account, prior: {client_account:?}");

        if client_account.locked {
            return Err(TransactionManagerError::AccountLocked);
        }

        let disputed_transaction = self.history.get(&r.tx);
        trace!("disputed_transaction: {disputed_transaction:?}");

        // Assuming that disputes, resolves, and chargebacks only apply to deposits,
        // which seems to make sense
        let Some(Transaction::Deposit(dep)) = disputed_transaction else {
            return Err(TransactionManagerError::DisputedTransactionDoesNotExist(
                r.tx,
            ));
        };

        // TODO: Is it possible for this to go negative? Should check
        client_account.available += dep.amount;
        client_account.held -= dep.amount;

        let _ = client_account.disputed_transactions.remove(&r.tx);

        trace!("client_account, after: {client_account:?}");

        Ok(())
    }
}

impl Default for TransactionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::balance::{ClientBalance, ClientBalanceRegistry};
    use crate::transactions::{Chargeback, Deposit, Dispute, Resolve, Transaction, Withdrawal};
    use std::collections::{HashMap, HashSet};
    use std::sync::Once;

    // TODO: Consider a few corner cases here
    // * rejecting transaction IDs we've already seen?
    // * rejecting if amount is negative?
    // * handling a withdrawal to an account we're seeing for the first time
    //   => Should this still create the account, but not place anything or not create the account?
    //   => For now, I'll go with creating the account

    static INIT: Once = Once::new();

    fn test_setup() {
        INIT.call_once(|| env_logger::init());
    }

    #[test]
    fn test_rejecting_duplicate_transaction_ids() {}

    #[test]
    fn test_negative_amount_deposit() {}

    #[test]
    fn test_simple_deposit_withdrawal() {
        test_setup();

        let mut tm = TransactionManager::new();

        let deposit = Transaction::Deposit(Deposit::new(1, 1, 32.0));
        let withdrawal = Transaction::Withdrawal(Withdrawal::new(1, 2, 20.0));

        tm.record_transaction(&deposit).unwrap();
        tm.record_transaction(&withdrawal).unwrap();

        let mut internal = HashMap::new();
        let client_1_balance = ClientBalance::new(12.0, 0.0, 12.0, false, HashSet::new());
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
            Transaction::Deposit(Deposit::new(2, 3, 2.0)),
            Transaction::Deposit(Deposit::new(1, 1, 32.0)),
            Transaction::Withdrawal(Withdrawal::new(2, 4, 1.0)),
            Transaction::Withdrawal(Withdrawal::new(1, 2, 20.0)),
        ];

        for transaction in &transactions {
            tm.record_transaction(transaction).unwrap();
        }
        let client_1_balance = ClientBalance::new(12.0, 0.0, 12.0, false, HashSet::new());
        let client_2_balance = ClientBalance::new(1.0, 0.0, 1.0, false, HashSet::new());
        let internal = HashMap::from([(1, client_1_balance), (2, client_2_balance)]);
        let expected_balances = ClientBalanceRegistry::load_registry(internal);

        let actual_balance = tm.retrieve_client_balances();

        assert_eq!(actual_balance, expected_balances);
    }

    #[test]
    fn test_insufficient_funds_for_withdrawal_first_operation() {
        test_setup();

        let mut tm = TransactionManager::new();

        let transaction = Transaction::Withdrawal(Withdrawal::new(2, 4, 200.0));

        let err = tm.record_transaction(&transaction).unwrap_err();
        assert_eq!(err, TransactionManagerError::InsufficientFunds(200.0));

        let client_2_balance = ClientBalance::new(0.0, 0.0, 0.0, false, HashSet::new());
        let internal = HashMap::from([(2, client_2_balance)]);

        let expected_balances = ClientBalanceRegistry::load_registry(internal);

        let actual_balance = tm.retrieve_client_balances();

        assert_eq!(actual_balance, expected_balances);
    }

    #[test]
    fn test_simple_dispute() {
        test_setup();

        let mut tm = TransactionManager::new();

        let transactions = vec![
            Transaction::Deposit(Deposit::new(1, 1, 32.0)),
            Transaction::Dispute(Dispute::new(1, 1)),
        ];

        for transaction in &transactions {
            tm.record_transaction(transaction).unwrap();
        }

        let client_1_balance = ClientBalance::new(0.0, 32.0, 32.0, false, HashSet::from([1]));
        let internal = HashMap::from([(1, client_1_balance)]);
        let expected_balances = ClientBalanceRegistry::load_registry(internal);

        let actual_balance = tm.retrieve_client_balances();

        assert_eq!(actual_balance, expected_balances);
    }

    #[test]
    fn test_simple_dispute_resolve() {
        test_setup();

        let mut tm = TransactionManager::new();

        let transactions = vec![
            Transaction::Deposit(Deposit::new(1, 1, 32.0)),
            Transaction::Dispute(Dispute::new(1, 1)),
            Transaction::Resolve(Resolve::new(1, 1)),
        ];

        for transaction in &transactions {
            tm.record_transaction(transaction).unwrap();
        }

        let client_1_balance = ClientBalance::new(32.0, 0.0, 32.0, false, HashSet::new());
        let internal = HashMap::from([(1, client_1_balance)]);
        let expected_balances = ClientBalanceRegistry::load_registry(internal);

        let actual_balance = tm.retrieve_client_balances();

        assert_eq!(actual_balance, expected_balances);
    }

    #[test]
    fn test_simple_dispute_chargeback() {
        test_setup();

        let mut tm = TransactionManager::new();

        let transactions = vec![
            Transaction::Deposit(Deposit::new(1, 1, 32.0)),
            Transaction::Dispute(Dispute::new(1, 1)),
            Transaction::Chargeback(Chargeback::new(1, 1)),
        ];

        for transaction in &transactions {
            tm.record_transaction(transaction).unwrap();
        }

        let client_1_balance = ClientBalance::new(0.0, 0.0, 0.0, true, HashSet::new());
        let internal = HashMap::from([(1, client_1_balance)]);
        let expected_balances = ClientBalanceRegistry::load_registry(internal);

        let actual_balance = tm.retrieve_client_balances();

        assert_eq!(actual_balance, expected_balances);
    }

    #[test]
    fn test_chargeback_blocks_transactions() {
        test_setup();

        let mut tm = TransactionManager::new();

        let transactions = vec![
            Transaction::Deposit(Deposit::new(1, 1, 32.0)),
            Transaction::Dispute(Dispute::new(1, 1)),
            Transaction::Chargeback(Chargeback::new(1, 1)),
        ];

        for transaction in &transactions {
            tm.record_transaction(transaction).unwrap();
        }

        let blocked_transaction = Transaction::Deposit(Deposit::new(1, 2, 42.0));
        let err = tm.record_transaction(&blocked_transaction).unwrap_err();

        assert_eq!(err, TransactionManagerError::AccountLocked);

        let client_1_balance = ClientBalance::new(0.0, 0.0, 0.0, true, HashSet::new());
        let internal = HashMap::from([(1, client_1_balance)]);
        let expected_balances = ClientBalanceRegistry::load_registry(internal);

        let actual_balance = tm.retrieve_client_balances();

        assert_eq!(actual_balance, expected_balances);
    }

    #[test]
    fn test_block_duplicated_transactions() {
        test_setup();

        let mut tm = TransactionManager::new();

        let transactions = vec![Transaction::Deposit(Deposit::new(1, 1, 32.0))];

        for transaction in &transactions {
            tm.record_transaction(transaction).unwrap();
        }

        let blocked_transaction = Transaction::Deposit(Deposit::new(1, 1, 42.0));
        let err = tm.record_transaction(&blocked_transaction).unwrap_err();

        assert_eq!(err, TransactionManagerError::DuplicateTransactionId(1));

        let client_1_balance = ClientBalance::new(32.0, 0.0, 32.0, false, HashSet::new());
        let internal = HashMap::from([(1, client_1_balance)]);
        let expected_balances = ClientBalanceRegistry::load_registry(internal);

        let actual_balance = tm.retrieve_client_balances();

        assert_eq!(actual_balance, expected_balances);
    }

    #[test]
    fn test_block_negative_amount() {
        test_setup();

        let mut tm = TransactionManager::new();

        let transactions = vec![Transaction::Deposit(Deposit::new(1, 1, 32.0))];

        for transaction in &transactions {
            tm.record_transaction(transaction).unwrap();
        }

        let blocked_transaction = Transaction::Deposit(Deposit::new(1, 2, -42.0));
        let err = tm.record_transaction(&blocked_transaction).unwrap_err();

        assert_eq!(err, TransactionManagerError::NegativeAmountNotAllowed);

        let client_1_balance = ClientBalance::new(32.0, 0.0, 32.0, false, HashSet::new());
        let internal = HashMap::from([(1, client_1_balance)]);
        let expected_balances = ClientBalanceRegistry::load_registry(internal);

        let actual_balance = tm.retrieve_client_balances();

        assert_eq!(actual_balance, expected_balances);
    }
}
