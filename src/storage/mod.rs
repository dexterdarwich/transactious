use std::collections::HashMap;

use crate::engine;
use engine::models::Account;
use engine::models::Transaction;
use engine::models::TransactionStore;

pub struct MemoryTransactionStore {
    accounts: HashMap<u16, Account>,
    pub transactions: HashMap<(u16, u32), Transaction>,
    disputes: HashMap<(u16, u32), Transaction>,
    resolves: HashMap<(u16, u32), Transaction>,
    chargebacks: HashMap<(u16, u32), Transaction>,
}

impl MemoryTransactionStore {
    pub fn new() -> Self {
        MemoryTransactionStore {
            accounts: HashMap::new(),
            transactions: HashMap::new(),
            disputes: HashMap::new(),
            resolves: HashMap::new(),
            chargebacks: HashMap::new(),
        }
    }
}

impl Default for MemoryTransactionStore {
    fn default() -> Self {
        MemoryTransactionStore::new()
    }
}

impl TransactionStore for MemoryTransactionStore {
    fn retrieve_account(&self, client: u16) -> Option<Account> {
        match self.accounts.get(&client) {
            None => None,
            Some(a) => Some(*a),
        }
    }

    fn save_account(&mut self, account: Account) {
        self.accounts.insert(account.client, account);
    }

    fn retrieve_transaction(&self, client: u16, tx: u32) -> Option<Transaction> {
        match self.transactions.get(&(client, tx)) {
            None => None,
            Some(t) => Some(*t),
        }
    }

    fn save_transaction(&mut self, txn: Transaction) {
        match txn {
            Transaction::Deposit {
                client,
                tx,
                amount: _,
            } => {
                self.transactions.insert((client, tx), txn);
            }
            Transaction::Withdrawal {
                client,
                tx,
                amount: _,
            } => {
                self.transactions.insert((client, tx), txn);
            }
            Transaction::Dispute { client, tx } => {
                self.disputes.insert((client, tx), txn);
            }
            Transaction::Resolve { client, tx } => {
                self.resolves.insert((client, tx), txn);
            }
            Transaction::Chargeback { client, tx } => {
                self.chargebacks.insert((client, tx), txn);
            }
        }
    }

    fn has_disputes(&self, client: u16, tx: u32) -> bool {
        self.disputes.get(&(client, tx)).is_some()
    }

    fn retrieve_all_accounts(&self) -> Vec<Account> {
        self.accounts.values().cloned().collect()
    }
}
