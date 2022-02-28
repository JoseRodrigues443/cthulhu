use criterion::{criterion_group, criterion_main, Criterion};
use cthulhu::engine::core::Engine;
use cthulhu::engine::database::Store;
use cthulhu::read;
use cthulhu::transaction::{Transaction, TransactionType};
use rust_decimal::Decimal;

fn happy_path() {
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

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("happy_path 20", |b| b.iter(happy_path));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
