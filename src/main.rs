use std::str::FromStr;
use serde::{Deserialize, Serialize};
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    input: String,
    #[clap(short, long)]
    output: String,
    #[clap(short, long)]
    bank: String,
}

enum Bank {
    SPankki,
    DanskeBank,
}

impl FromStr for Bank {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "s-pankki" => Ok(Bank::SPankki),
            "danske-bank" => Ok(Bank::DanskeBank),
            _ => Err(format!("Unknown bank: {}", s)),
        }
    }
}

fn main() {
    let args = Args::parse();

    let mut writer = csv::WriterBuilder::new()
        .delimiter(b';')
        .from_path(args.output.as_str())
        .expect("Failed to open file");

    let bank = Bank::from_str(args.bank.as_str()).expect("Failed to parse bank");

    deserialize_bank(bank, args.input.as_str())
        .iter()
        .for_each(|record| {
            writer.serialize(record).expect("Failed to serialize record");
        });

    let _ = writer.flush();
}

fn deserialize_bank(bank: Bank, input_path: &str) -> Vec<YnabImportRow> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(true)
        .flexible(true)
        .from_path(input_path)
        .expect("Failed to open file");
    match bank {
        Bank::SPankki => {
            // read csv file with semicolon as separator
            reader
                .deserialize::<SPankkiAccountStatementRow>()
                .flatten()
                .map(YnabImportRow::from)
                .collect()
        }
        Bank::DanskeBank => {
            reader
                .deserialize::<DanskeBankAccountStatementRow>()
                .flatten()
                .filter_map(|row| row.try_into().ok())
                .collect()
        }
    }
}

#[derive(Debug, Deserialize)]
struct SPankkiAccountStatementRow {
    #[serde(rename = "Maksupäivä")]
    date: String,
    #[serde(rename = "Saajan nimi")]
    payee: String,
    #[serde(rename = "Viesti")]
    memo: String,
    #[serde(rename = "Summa")]
    amount: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct DanskeBankAccountStatementRow {
    date: String,
    class: String,
    subclass: String,
    payee: String,
    amount: String,
    balance: String,
    status: String,
    check: String,
}

#[derive(Debug, Serialize)]
struct YnabImportRow {
    #[serde(rename = "Date")]
    date: String,
    #[serde(rename = "Payee")]
    payee: String,
    #[serde(rename = "Memo")]
    memo: String,
    #[serde(rename = "Amount")]
    amount: String,
}

impl From<SPankkiAccountStatementRow> for YnabImportRow {
    fn from(row: SPankkiAccountStatementRow) -> Self {
        YnabImportRow {
            date: row.date,
            payee: row.payee,
            memo: row.memo.replace("'", ""),
            amount: row.amount.replace(",", ".").replace("+", "")
        }
    }
}

impl TryFrom<DanskeBankAccountStatementRow> for YnabImportRow {
    type Error = String;

    fn try_from(value: DanskeBankAccountStatementRow) -> Result<Self, Self::Error> {
        match value.status.as_str() {
            "Toteutunut" => Ok(YnabImportRow {
                date: value.date,
                payee: value.payee.trim().to_string(),
                memo: format!("{}: {}", value.class.trim(), value.subclass.trim()),
                amount: value.amount.replace(",", ".").replace("+", "")
            }),
            _ => Err(format!("Transaction not executed: {}", value.payee))
        }

    }
}