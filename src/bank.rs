use std::str::FromStr;
use serde::{Deserialize};

pub enum Bank {
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

#[derive(Debug, Deserialize)]
pub struct SPankkiAccountStatementRow {
    #[serde(rename = "Maksupäivä")]
    pub date: String,
    #[serde(rename = "Saajan nimi")]
    pub payee: String,
    #[serde(rename = "Viesti")]
    pub memo: String,
    #[serde(rename = "Summa")]
    pub amount: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct DanskeBankAccountStatementRow {
    #[serde(rename = "Pvm")]
    pub date: String,
    #[serde(rename = "Luokka")]
    pub class: String,
    #[serde(rename = "Alaluokka")]
    pub subclass: String,
    #[serde(rename = "Saaja/Maksaja")]
    pub payee: String,
    #[serde(rename = "M��r�")]
    pub amount: String,
    #[serde(rename = "Saldo")]
    pub balance: String,
    #[serde(rename = "Tila")]
    pub status: String,
    #[serde(rename = "Tarkastus")]
    pub check: String,
}
