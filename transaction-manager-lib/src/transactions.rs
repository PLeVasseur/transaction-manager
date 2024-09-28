use serde::de::{self, Deserializer};
use serde::Deserialize;

#[derive(Clone, Debug)]
pub struct Deposit {
    pub client: u16,
    pub tx: u32,
    pub amount: f64,
}

impl Deposit {
    pub fn new(client: u16, tx: u32, amount: f64) -> Self {
        Self { client, tx, amount }
    }
}

#[derive(Clone, Debug)]
pub struct Withdrawal {
    pub client: u16,
    pub tx: u32,
    pub amount: f64,
}

impl Withdrawal {
    pub fn new(client: u16, tx: u32, amount: f64) -> Self {
        Self { client, tx, amount }
    }
}

#[derive(Clone, Debug)]
pub struct Dispute {
    pub client: u16,
    pub tx: u32,
}

impl Dispute {
    pub fn new(client: u16, tx: u32) -> Self {
        Self { client, tx }
    }
}

#[derive(Clone, Debug)]
pub struct Resolve {
    pub client: u16,
    pub tx: u32,
}

impl Resolve {
    pub fn new(client: u16, tx: u32) -> Self {
        Self { client, tx }
    }
}

#[derive(Clone, Debug)]
pub struct Chargeback {
    pub client: u16,
    pub tx: u32,
}

impl Chargeback {
    pub fn new(client: u16, tx: u32) -> Self {
        Self { client, tx }
    }
}

#[derive(Clone, Debug)]
pub enum Transaction {
    Deposit(Deposit),
    Withdrawal(Withdrawal),
    Dispute(Dispute),
    Resolve(Resolve),
    Chargeback(Chargeback),
}

impl<'de> Deserialize<'de> for Transaction {
    fn deserialize<D>(deserializer: D) -> Result<Transaction, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct TransactionRecord {
            #[serde(rename = "type")]
            tx_type: String,
            client: u16,
            tx: u32,
            // Optional, since not all types use the transaction amount
            amount: Option<f64>,
        }

        let record = TransactionRecord::deserialize(deserializer)?;

        match record.tx_type.as_str() {
            "deposit" => {
                if let Some(amount) = record.amount {
                    Ok(Transaction::Deposit(Deposit {
                        client: record.client,
                        tx: record.tx,
                        amount,
                    }))
                } else {
                    Err(de::Error::custom("Missing amount for deposit"))
                }
            }
            "withdrawal" => {
                if let Some(amount) = record.amount {
                    Ok(Transaction::Withdrawal(Withdrawal {
                        client: record.client,
                        tx: record.tx,
                        amount,
                    }))
                } else {
                    Err(de::Error::custom("Missing amount for withdrawal"))
                }
            }
            "dispute" => Ok(Transaction::Dispute(Dispute {
                client: record.client,
                tx: record.tx,
            })),
            "resolve" => Ok(Transaction::Resolve(Resolve {
                client: record.client,
                tx: record.tx,
            })),
            "chargeback" => Ok(Transaction::Chargeback(Chargeback {
                client: record.client,
                tx: record.tx,
            })),
            _ => Err(de::Error::custom("Unknown transaction type")),
        }
    }
}
