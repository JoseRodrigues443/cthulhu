use crate::transaction::{Report, Transaction};
use rust_decimal::Decimal;
use std::collections::HashMap;

#[derive(Default, Debug, Clone)]
pub struct Store {
    pub by_client_id: HashMap<u16, Report>,
    pub by_transaction_id: HashMap<u32, Transaction>,
}

impl Store {
    pub fn create_empty_report(client_id: u16) -> Report {
        Report {
            client: client_id,
            available: Decimal::new(0, 0),
            held: Decimal::new(0, 0),
            locked: false,
            total: Decimal::new(0, 0),
        }
    }
}
