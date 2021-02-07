use std::collections::hash_map::Values;

#[derive(Debug)]
pub enum Transaction {
    Deposit { client: u16, tx: u32, amount: u128 },
    Withdrawal { client: u16, tx: u32, amount: u128 },
    Dispute { client: u16, tx: u32 },
    Resolve { client: u16, tx: u32 },
    Chargeback { client: u16, tx: u32 },
}

pub struct Account {
    pub client: u16,
    pub available: u128,
    pub held: u128,
    pub total: u128,
    pub locked: bool,
}

pub trait TransactionStore {
    fn retrieve_account(&mut self, client: u16) -> &mut Account;
    fn save_account(&mut self, account: Account);
    fn retrieve_transaction(&self, client: u16, tx: u32) -> Option<&Transaction>;
    fn save_transaction(&mut self, transaction: Transaction);
    fn has_disputes(&self, client: u16, tx: u32) -> bool;
    fn retrieve_all_accounts(&self) -> Values<'_, u16, Account>;
}
