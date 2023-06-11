mod bank;
mod ynab;

use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use clap::Parser;
use crate::bank::{Bank, DanskeBankAccountStatementRow, SPankkiAccountStatementRow};
use crate::ynab::YnabImportRow;

#[derive(Parser)]
struct Args {
    /// Path to input CSV
    #[clap(short, long)]
    input: String,
    /// Path to output CSV
    #[clap(short, long)]
    output: String,
    /// Bank: "s-pankki" or "danske"
    #[clap(short, long)]
    bank: String,
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
    let mut file = File::open(input_path).expect("Failed to open file");
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).expect("Failed to read file");
    let file_contents = String::from_utf8_lossy(buf.as_slice()).to_string();

    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(true)
        .flexible(true)
        .from_reader(file_contents.as_bytes());
    match bank {
        Bank::SPankki => {
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
