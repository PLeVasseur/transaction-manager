use std::collections::HashMap;

// TODO: Consider _not_ implementing Clone here when I've
// better fleshed out how to return a reference to this
// from TransactionManager
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ClientBalanceRegistry {
    pub client_balances: HashMap<u16, ClientBalance>
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ClientBalance {
    pub available: f64,
    pub held: f64,
    pub total: f64,
    pub locked: bool
}

impl ClientBalance {
    pub fn new(available: f64, held: f64, total: f64, locked: bool) -> Self {
        Self {
            available,
            held,
            total,
            locked
        }
    }
}

impl ClientBalanceRegistry {
    pub fn new() -> Self {
        Self {
            client_balances: HashMap::new()
        }
    }

    #[cfg(test)]
    pub fn load_registry(client_balances: HashMap<u16, ClientBalance>) -> Self {
        Self {
            client_balances
        }
    }

    pub fn to_csv(&self) -> String {
        let mut csv_data = String::new();
        csv_data.push_str("client,available,held,total,locked\n"); // CSV header
        
        for (client_id, balance) in &self.client_balances {
            let row = format!("{},{},{},{},{}\n", 
                              client_id, 
                              balance.available, 
                              balance.held, 
                              balance.total, 
                              balance.locked);
            csv_data.push_str(&row);
        }
        
        csv_data
    }
}
