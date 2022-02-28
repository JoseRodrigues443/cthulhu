use crate::engine::actions::Action;
use crate::engine::database::Store;
use crate::engine::registry::Registry;
use crate::transaction::{Transaction, TransactionType};

use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub trait Engine {
    fn process_transaction(&mut self, trans: &Transaction) -> Result<()>;
}

impl Engine for Store {
    fn process_transaction(&mut self, trans: &Transaction) -> Result<()> {
        let cid = match trans.client {
            Some(v) => v,
            None => return Err("Need client id on the transaction".into()),
        };
        let report = self.get_report_by_id(cid);
        if let Some(cas) = report {
            if cas.locked {
                return Ok(());
            }
        };

        match trans.transaction_type {
            TransactionType::Deposit => {
                self.deposit(trans)?;
            }
            TransactionType::Withdrawal => {
                self.deposit(trans)?;
            }
            TransactionType::Dispute => {
                self.deposit(trans)?;
            }
            TransactionType::Resolve => {
                self.deposit(trans)?;
            }
            TransactionType::Chargeback => {
                self.deposit(trans)?;
            }
        };
        Ok(())
    }
}
