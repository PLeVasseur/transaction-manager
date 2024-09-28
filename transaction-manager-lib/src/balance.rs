use std::collections::HashMap;

pub struct ClientBalanceRegistry {
    pub client_balances: HashMap<u16, ClientBalance>
}

pub struct ClientBalance {
    pub available: f64,
    pub held: f64,
    pub total: f64,
    pub locked: f64
}

impl ClientBalanceRegistry {
    pub fn new() -> Self {
        Self {
            client_balances: HashMap::new()
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
