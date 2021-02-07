use std::collections::HashMap;

use crate::engine;
use engine::models::Account;
use engine::models::Transaction;
use engine::models::TransactionStore;
use std::collections::hash_map::Values;

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
    fn retrieve_account(&mut self, client: u16) -> &mut Account {
        let new_account = Account {
            client,
            available: 0,
            held: 0,
            total: 0,
            locked: false,
        };
        self.accounts.entry(client).or_insert(new_account)
    }

    fn save_account(&mut self, account: Account) {
        self.accounts.insert(account.client, account);
    }

    fn retrieve_transaction(&self, client: u16, tx: u32) -> Option<&Transaction> {
        self.transactions.get(&(client, tx))
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

    fn retrieve_all_accounts(&self) -> Values<'_, u16, Account> {
        self.accounts.values()
    }
}
