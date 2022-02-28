use cthulhu::engine::core::Engine;
use cthulhu::engine::database::Store;
use cthulhu::read;
use cthulhu::transaction::{Transaction, TransactionType};
use rust_decimal::Decimal;

#[test]
fn read_original_example() {
    let transaction = Transaction {
        client: Some(1),
        amount: Some(Decimal::new(202, 2)),
        transaction_type: TransactionType::Deposit,
        tx: Some(1),
    };
    let mut store = Store::default();
    store.process_transaction(&transaction).unwrap();
    read::write_ledger(std::io::stdout(), store).unwrap();
}
