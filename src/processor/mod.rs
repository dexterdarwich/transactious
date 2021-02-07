use serde::Deserialize;
use std::env;
use std::ffi::OsString;
use std::error::Error;
use std::fmt;

use crate::engine;
use engine::models::Transaction;
use engine::ProcessingEngine;

use crate::storage;
use storage::MemoryTransactionStore;

#[derive(Debug, Deserialize)]
struct Record<'a> {
    r#type: &'a str,
    client: u16,
    tx: u32,
    amount: Option<&'a str>,
}

#[derive(Debug)]
struct MyError(String);

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for MyError {}

pub fn process() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg();
    let txn_store = MemoryTransactionStore::new();
    let processing_engine = ProcessingEngine::new(txn_store);
    let mut reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .flexible(true)
        .from_path(file_path)?;
    let headers = reader.headers()?.clone();
    let mut raw_record = csv::StringRecord::new();

    while reader.read_record(&mut raw_record)? {
        let record: Record = raw_record.deserialize(Some(&headers))?;
        let txn = record_to_txn_enum(&record)?;
        processing_engine.process(txn);
    }

    // Print the Accounts
    processing_engine.print_all_account();
    Ok(())
}

fn record_to_txn_enum(record: &Record) -> Result<Transaction, Box<dyn Error>> {
    match record.r#type {
        "deposit" => {
            let amount = get_amount(&record)?;
            let trx = Transaction::Deposit {
                client: record.client,
                tx: record.tx,
                amount,
            };
            Ok(trx)
        }
        "withdrawal" => {
            let amount = get_amount(&record)?;
            let trx = Transaction::Withdrawal {
                client: record.client,
                tx: record.tx,
                amount,
            };
            Ok(trx)
        }
        "dispute" => {
            let trx = Transaction::Dispute {
                client: record.client,
                tx: record.tx,
            };
            Ok(trx)
        }
        "resolve" => {
            let trx = Transaction::Resolve {
                client: record.client,
                tx: record.tx,
            };
            Ok(trx)
        }
        "chargeback" => {
            let trx = Transaction::Chargeback {
                client: record.client,
                tx: record.tx,
            };
            Ok(trx)
        }
        _ => Err(Box::new(MyError("Unsupported transaction type.".into()))),
    }
}

fn get_amount(record: &Record) -> Result<u128, Box<dyn Error>> {
    str_to_amount(record.amount.unwrap_or_else(|| panic!("Amount is empty for {:?}.", record)))
}

fn str_to_amount(amount_str: &str) -> Result<u128, Box<dyn Error>> {
    let split: Vec<&str> = amount_str.split('.').collect();
    let whole = split[0].parse::<u128>()? * 10000u128;
    let decimal = format!("{:0<4}", split[1]).parse::<u128>()?;
    Ok(whole + decimal)
}

pub(crate) fn amount_to_string(amount: u128) -> String {
    format!("{}.{:0>4}", amount / 10000, amount % 10000)
}

fn get_first_arg() -> OsString {
    env::args_os().nth(1).expect("Expected input file as the first argument, but got none.")
}

#[cfg(test)]
mod tests {
    use crate::processor::amount_to_string;
    use crate::processor::str_to_amount;
    #[test]
    fn string_to_u128() {
        assert_eq!(str_to_amount("1.2").unwrap(), 12000);
        assert_eq!(str_to_amount("123.201").unwrap(), 1232010);
        assert_eq!(str_to_amount("1.0001").unwrap(), 10001);
        assert_eq!(str_to_amount("1.0").unwrap(), 10000);
        assert_eq!(str_to_amount("0.01").unwrap(), 100);
        assert_eq!(str_to_amount("0.0").unwrap(), 0);
    }

    #[test]
    fn u128_to_string() {
        assert_eq!(amount_to_string(10000u128), "1.0000");
        assert_eq!(amount_to_string(11000u128), "1.1000");
        assert_eq!(amount_to_string(1000u128), "0.1000");
        assert_eq!(amount_to_string(70522u128), "7.0522");
    }
}
