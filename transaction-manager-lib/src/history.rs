use std::collections::HashMap;
use crate::transactions::Transaction;
use std::ops::Deref;

pub struct TransactionHistory {
    history: HashMap<u32, Transaction>
}

impl TransactionHistory {
    pub fn new() -> Self {
        Self {
            history: HashMap::new()
        }
    }
}

impl Deref for TransactionHistory {
    type Target = HashMap<u32, Transaction>;

    fn deref(&self) -> &Self::Target {
        &self.history
    }
}

