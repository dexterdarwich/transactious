pub mod models;

use models::Transaction;
use models::TransactionStore;
use std::cell::{RefCell};

use crate::processor::amount_to_string;

pub struct ProcessingEngine<T> {
    store: RefCell<T>,
}

impl<T: TransactionStore> ProcessingEngine<T> {
    pub fn new(t: T) -> Self
        where
            T: TransactionStore,
    {
        ProcessingEngine { store: RefCell::new(t) }
    }

    pub fn process(&self, txn: Transaction) {
        match txn {
            Transaction::Deposit {
                client,
                tx: _,
                amount,
            } => {
                let store = &mut *self.store.borrow_mut();
                let account = store.retrieve_account(client);
                account.available += amount;
                account.total += amount;
                store.save_transaction(txn);
            }
            Transaction::Withdrawal {
                client,
                tx: _,
                amount,
            } => {
                let store = &mut *self.store.borrow_mut();
                let account = store.retrieve_account(client);
                if account.available >= amount {
                    account.available -= amount;
                    account.total -= amount;
                    store.save_transaction(txn);
                }
            }
            Transaction::Dispute { client, tx } => {
                let store = &mut *self.store.borrow_mut();
                match store.retrieve_transaction(client, tx) {
                    None => (),
                    Some(txn) => {
                        let amount = match txn {
                            Transaction::Deposit {
                                client: _,
                                tx: _,
                                amount,
                            } => *amount,
                            _ => 0,
                        };
                        if amount > 0 {
                            let account = store.retrieve_account(client);
                            account.available -= amount;
                            account.held += amount;
                        }
                    }
                }
            }
            Transaction::Resolve { client, tx } => {
                let store = &mut *self.store.borrow_mut();
                if !store.has_disputes(client, tx) {
                    match store.retrieve_transaction(client, tx) {
                        None => (),
                        Some(txn) => {
                            let amount = match txn {
                                Transaction::Deposit {
                                    client: _,
                                    tx: _,
                                    amount,
                                } => *amount,
                                _ => 0,
                            };
                            if amount > 0 {
                                let account = store.retrieve_account(client);
                                account.available += amount;
                                account.held -= amount;
                            }
                        }
                    }
                }
            }
            Transaction::Chargeback { client, tx } => {
                let mut store = self.store.borrow_mut();
                if !store.has_disputes(client, tx) {
                    match store.retrieve_transaction(client, tx) {
                        None => (),
                        Some(txn) => {
                            let amount = match txn {
                                Transaction::Deposit {
                                    client: _,
                                    tx: _,
                                    amount,
                                } => *amount,
                                _ => 0,
                            };
                            if amount > 0 {
                                let account = store.retrieve_account(client);
                                account.total -= amount;
                                account.held -= amount;
                                account.locked = true;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn print_all_account(&self) {
        println!("client,available,held,total,locked");
        for account in (*self.store.borrow()).retrieve_all_accounts() {
            println!(
                "{},{},{},{},{}",
                account.client,
                amount_to_string(account.available),
                amount_to_string(account.held),
                amount_to_string(account.total),
                account.locked
            );
        }
    }
}
