use clap::Parser;
use cthulhu::engine::core::Engine;
use cthulhu::engine::database::Store;
use cthulhu::{args::Args, read};
use std::io;

fn main() -> io::Result<()> {
    let Args { input_file } = Args::parse();

    let file = std::fs::File::open(input_file)?;

    let mut store = Store::default();

    for txn in
        read::read_tx(file).map(|rtxr| rtxr.map_err(|e| io::Error::new(io::ErrorKind::Other, e)))
    {
        let txn = txn?;
        if let Err(e) = store.process_transaction(&txn) {
            eprintln!("{} in transaction:\n{:#?}", e, txn);
        }
    }

    read::write_ledger(std::io::stdout(), store).unwrap();

    Ok(())
}
