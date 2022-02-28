use crate::engine::actions::Entry::Occupied;
use crate::engine::actions::Entry::Vacant;
use std::collections::hash_map::Entry;

use crate::engine::database::Store;
use crate::transaction::Transaction;
use rust_decimal::Decimal;

use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub trait Action {
    fn deposit(&mut self, trans: &Transaction) -> Result<()>;
    fn withdrawal(&mut self, trans: &Transaction) -> Result<()>;
    fn dispute(&mut self, trans: &Transaction) -> Result<()>;
    fn resolve(&mut self, trans: &Transaction) -> Result<()>;
    fn chargeback(&mut self, trans: &Transaction) -> Result<()>;
}

impl Action for Store {
    fn deposit(&mut self, trans: &Transaction) -> Result<()> {
        let cid = match trans.client {
            Some(v) => v,
            None => return Err("need client id from transaction".into()),
        };
        let tid = match trans.tx {
            Some(v) => v,
            None => return Err("need transaction id from transaction".into()),
        };
        let amount = match trans.amount {
            Some(v) => v,
            None => Decimal::new(0, 0),
        };
        match self.by_transaction_id.entry(tid) {
            Occupied(_) => {}
            Vacant(entry) => {
                entry.insert(trans.clone());
            }
        }
        match self.by_client_id.entry(cid) {
            Occupied(mut entry) => {
                let mut acct_status = entry.get_mut();
                let current_available = acct_status.available;
                acct_status.available = current_available + amount;
            }
            Vacant(entry) => {
                let mut acct_status = Self::create_empty_report(cid);
                acct_status.available = amount;
                entry.insert(acct_status);
            }
        }
        Ok(())
    }

    fn withdrawal(&mut self, trans: &Transaction) -> Result<()> {
        let cid = match trans.client {
            Some(v) => v,
            None => return Err("need client id from transaction".into()),
        };
        let tid = match trans.tx {
            Some(v) => v,
            None => return Err("need transaction id from transaction".into()),
        };
        let amount = match trans.amount {
            Some(v) => v,
            None => return Err("need amount from transaction".into()),
        };
        match self.by_transaction_id.entry(tid) {
            Occupied(_) => {}
            Vacant(entry) => {
                entry.insert(trans.clone());
            }
        }
        match self.by_client_id.entry(cid) {
            Occupied(mut entry) => {
                let mut acct_status = entry.get_mut();
                let current_available = acct_status.available;
                if amount > current_available {
                    return Err("Insufficient funds".into());
                }
                acct_status.available = current_available - amount;
            }
            Vacant(_) => {
                return Err("Insufficient funds, non existent by client id".into());
            }
        }
        Ok(())
    }

    fn dispute(&mut self, trans: &Transaction) -> Result<()> {
        let cid = match trans.client {
            Some(v) => v,
            None => return Err("need client id from transaction".into()),
        };
        let tid = match trans.tx {
            Some(v) => v,
            None => return Err("need transaction id from transaction".into()),
        };
        match self.by_transaction_id.entry(tid) {
            Occupied(client_transaction) => {
                let ct = client_transaction.get();
                let clientid = ct.client.unwrap_or(0);
                if clientid == cid {
                    // only proceed if transaction is for the right client id indicated in dispute
                    match self.by_client_id.entry(cid) {
                        Occupied(mut client_account_status) => {
                            let cas = client_account_status.get_mut();
                            let cat_amount_val = match ct.amount {
                                Some(v) => v,
                                None => Decimal::new(0, 0),
                            };
                            if cat_amount_val <= cas.available {
                                cas.available -= cat_amount_val;
                                cas.held += cat_amount_val;
                            }
                        }
                        Vacant(_) => {}
                    }
                } else {
                }
            }
            Vacant(_) => {}
        }
        Ok(())
    }

    fn resolve(&mut self, trans: &Transaction) -> Result<()> {
        let cid = match trans.client {
            Some(v) => v,
            None => return Err("need client id from transaction".into()),
        };
        let tid = match trans.tx {
            Some(v) => v,
            None => return Err("need transaction id from transaction".into()),
        };
        match self.by_transaction_id.entry(tid) {
            Occupied(client_transaction) => {
                let ct = client_transaction.get();
                let clientid = ct.client.unwrap_or(0);
                if clientid == cid {
                    // only proceed if transaction is for the right client id indicated in dispute
                    match self.by_client_id.entry(cid) {
                        Occupied(mut client_account_status) => {
                            let cas = client_account_status.get_mut();
                            let cat_amount_val = match ct.amount {
                                Some(v) => v,
                                None => Decimal::new(0, 0),
                            };
                            if cat_amount_val <= cas.held {
                                // must have enough in help to resolve amount
                                cas.available += cat_amount_val;
                                cas.held -= cat_amount_val;
                            }
                        }
                        Vacant(_) => {}
                    }
                } else {
                }
            }
            Vacant(_) => {}
        }
        Ok(())
    }

    fn chargeback(&mut self, trans: &Transaction) -> Result<()> {
        let cid = match trans.client {
            Some(v) => v,
            None => return Err("need client id from transaction".into()),
        };
        let tid = match trans.tx {
            Some(v) => v,
            None => return Err("need transaction id from transaction".into()),
        };
        match self.by_transaction_id.entry(tid) {
            Occupied(client_transaction) => {
                let ct = client_transaction.get();
                let clientid = ct.client.unwrap_or(0);
                if clientid == cid {
                    // only proceed if transaction is for the right client id indicated in dispute
                    match self.by_client_id.entry(cid) {
                        Occupied(mut client_account_status) => {
                            let mut cas = client_account_status.get_mut();
                            let cat_amount_val = match ct.amount {
                                Some(v) => v,
                                None => Decimal::new(0, 0),
                            };
                            if cat_amount_val <= cas.available {
                                // must have enough in available to chargeback amount (i.e. withdraw from account)
                                cas.available -= cat_amount_val;
                                cas.locked = true; // always freeze account after chargeback
                            }
                        }
                        Vacant(_) => {}
                    }
                } else {
                }
            }
            Vacant(_) => {}
        }
        Ok(())
    }
}
