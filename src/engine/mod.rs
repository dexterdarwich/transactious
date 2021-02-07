pub mod models;
use models::Account;
use models::Transaction;
use models::TransactionStore;

pub struct ProcessingEngine<'a, T> {
    store: &'a mut T,
}

impl<'a, T: TransactionStore> ProcessingEngine<'a, T> {
    pub fn new(t: &'a mut T) -> Self
    where
        T: TransactionStore,
    {
        ProcessingEngine { store: t }
    }

    pub fn process(&mut self, txn: &Transaction) {
        match txn {
            Transaction::Deposit {
                client,
                tx: _,
                amount,
            } => {
                let mut account = match self.store.retrieve_account(*client) {
                    Some(account) => account,
                    None => {
                        let account = Account {
                            client: *client,
                            available: 0,
                            held: 0,
                            total: 0,
                            locked: false,
                        };
                        account
                    }
                };
                account.available += amount;
                account.total += amount;
                self.store.save_account(account);
                self.store.save_transaction(*txn);
            }
            Transaction::Withdrawal {
                client,
                tx: _,
                amount,
            } => match self.store.retrieve_account(*client) {
                None => (),
                Some(acct) => {
                    if acct.available >= *amount {
                        let mut account = acct;
                        account.available -= amount;
                        account.total -= amount;
                        self.store.save_account(account);
                        self.store.save_transaction(*txn);
                    }
                }
            },
            Transaction::Dispute { client, tx } => {
                match self.store.retrieve_transaction(*client, *tx) {
                    None => (),
                    Some(txn) => {
                        let amount = match txn {
                            Transaction::Deposit {
                                client: _,
                                tx: _,
                                amount,
                            } => amount,
                            _ => 0,
                        };
                        if amount > 0 {
                            match self.store.retrieve_account(*client) {
                                None => (),
                                Some(acct) => {
                                    let mut account = acct;
                                    account.available -= amount;
                                    account.held += amount;
                                    self.store.save_account(account);
                                }
                            }
                        }
                    }
                }
            }
            Transaction::Resolve { client, tx } => {
                if !self.store.has_disputes(*client, *tx) {
                    match self.store.retrieve_transaction(*client, *tx) {
                        None => (),
                        Some(txn) => {
                            let amount = match txn {
                                Transaction::Deposit {
                                    client: _,
                                    tx: _,
                                    amount,
                                } => amount,
                                _ => 0,
                            };
                            if amount > 0 {
                                match self.store.retrieve_account(*client) {
                                    None => (),
                                    Some(acct) => {
                                        let mut account = acct;
                                        account.available += amount;
                                        account.held -= amount;
                                        self.store.save_account(account);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Transaction::Chargeback { client, tx } => {
                if !self.store.has_disputes(*client, *tx) {
                    match self.store.retrieve_transaction(*client, *tx) {
                        None => (),
                        Some(txn) => {
                            let amount = match txn {
                                Transaction::Deposit {
                                    client: _,
                                    tx: _,
                                    amount,
                                } => amount,
                                _ => 0,
                            };
                            if amount > 0 {
                                match self.store.retrieve_account(*client) {
                                    None => (),
                                    Some(acct) => {
                                        let mut account = acct;
                                        account.total -= amount;
                                        account.held -= amount;
                                        account.locked = true;
                                        self.store.save_account(account);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
