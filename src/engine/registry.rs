use crate::engine::database::Store;
use crate::transaction::Report;
use crate::transaction::Transaction;
use rust_decimal::Decimal;
use std::collections::hash_map::Entry;

use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub trait Registry {
    fn get_report_by_id(&mut self, client_id: u16) -> Option<&Report>;
    fn get_report_entry_by_id(&mut self, cid: u16) -> Entry<'_, u16, Report>;
    fn get_transaction_by_id(&mut self, tx_id: u32) -> Entry<'_, u32, Transaction>;
    fn get_funds_available(&self, client_id: u16) -> Result<Decimal>;
    fn get_funds_held(&self, client_id: u16) -> Result<Decimal>;
    fn get_funds_total(&self, client_id: u16) -> Result<Decimal>;
}

impl Registry for Store {
    fn get_report_by_id(&mut self, cid: u16) -> Option<&Report> {
        self.by_client_id.get(&cid)
    }

    fn get_report_entry_by_id(&mut self, cid: u16) -> Entry<'_, u16, Report> {
        self.by_client_id.entry(cid)
    }

    fn get_transaction_by_id(&mut self, tx_id: u32) -> Entry<'_, u32, Transaction> {
        self.by_transaction_id.entry(tx_id)
    }

    fn get_funds_available(&self, client_id: u16) -> Result<Decimal> {
        let available = match self.by_client_id.get(&client_id) {
            Some(v) => v.available,
            None => return Err("account status not initialized".into()),
        };
        Ok(available)
    }
    fn get_funds_held(&self, client_id: u16) -> Result<Decimal> {
        let available = match self.by_client_id.get(&client_id) {
            Some(v) => v.available,
            None => return Err("account status not initialized".into()),
        };
        Ok(available)
    }

    fn get_funds_total(&self, client_id: u16) -> Result<Decimal> {
        let cas = match self.by_client_id.get(&client_id) {
            Some(v) => v,
            None => return Err("account status not initialized".into()),
        };
        let total = cas.available + cas.held;
        Ok(total)
    }
}
