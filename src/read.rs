use crate::constants::DECIMAL_PRECISION;
use crate::engine::database::Store;
use crate::transaction::Report;

use crate::transaction::Transaction;
use csv::{ReaderBuilder, Trim};

use std::io;

pub fn read_tx(reader: impl io::Read) -> impl Iterator<Item = Result<Transaction, csv::Error>> {
    let rdr = ReaderBuilder::new()
        .flexible(true)
        .trim(Trim::All)
        .from_reader(reader);
    rdr.into_deserialize()
}

pub fn write_ledger(writer: impl io::Write, registry: Store) -> Result<(), csv::Error> {
    let mut wtr = csv::Writer::from_writer(writer);

    let all_clients = registry.by_client_id;
    for (_, cas) in all_clients {
        wtr.serialize(Report {
            client: cas.client,
            available: cas.available.round_dp(DECIMAL_PRECISION),
            held: cas.held.round_dp(DECIMAL_PRECISION),
            total: cas.total.round_dp(DECIMAL_PRECISION),
            locked: cas.locked,
        })?
    }
    Ok(())
}
