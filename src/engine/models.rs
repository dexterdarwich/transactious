#[derive(Debug, Clone, Copy)]
pub enum Transaction {
    Deposit { client: u16, tx: u32, amount: u128 },
    Withdrawal { client: u16, tx: u32, amount: u128 },
    Dispute { client: u16, tx: u32 },
    Resolve { client: u16, tx: u32 },
    Chargeback { client: u16, tx: u32 },
}

#[derive(Clone, Copy)]
pub struct Account {
    pub client: u16,
    pub available: u128,
    pub held: u128,
    pub total: u128,
    pub locked: bool,
}

pub trait TransactionStore {
    fn retrieve_account(&self, client: u16) -> Option<Account>;
    fn save_account(&mut self, account: Account);
    fn retrieve_transaction(&self, client: u16, tx: u32) -> Option<Transaction>;
    fn save_transaction(&mut self, transaction: Transaction);
    fn has_disputes(&self, client: u16, tx: u32) -> bool;
    fn retrieve_all_accounts(&self) -> Vec<Account>;
}
