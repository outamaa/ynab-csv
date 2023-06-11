use serde::Serialize;
use crate::bank::{DanskeBankAccountStatementRow, SPankkiAccountStatementRow};

#[derive(Debug, Serialize)]
pub struct YnabImportRow {
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "Payee")]
    pub payee: String,
    #[serde(rename = "Memo")]
    pub memo: String,
    #[serde(rename = "Amount")]
    pub amount: String,
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