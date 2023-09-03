pub(crate) mod oauth;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Serialize, Deserialize)]
pub enum HttpMethod {
    Get,
    Post,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Scope {
    #[serde(rename = "login")]
    Login,
    #[serde(rename = "inquiry")]
    Inquiry,
    #[serde(rename = "transfer")]
    Transfer,
    #[serde(rename = "cardinfo")]
    CardInfo,
    #[serde(rename = "fintechinfo")]
    FintechInfo,
    #[serde(rename = "insuinfo")]
    InsuInfo,
    #[serde(rename = "loaninfo")]
    LoanInfo,
}

#[repr(i32)]
#[derive(Debug, Serialize_repr, Deserialize_repr)]
pub enum AuthType {
    First = 0,
    Ignore = 2,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Lang {
    #[serde(rename = "kor")]
    Kor,
    #[serde(rename = "eng")]
    Eng,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RegisterKind {
    #[serde(rename = "A")]
    Account, // 계좌
    #[serde(rename = "C")]
    Card, // 카드(신용/체크)
    #[serde(rename = "F")]
    Pays, // 선불
    #[serde(rename = "I")]
    Insurance, // 보험
    #[serde(rename = "L")]
    Loan, // 캐피탈(대출/리스)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientDeviceType {
    #[serde(rename = "PC")]
    PC,
    #[serde(rename = "AD")]
    Andriod,
    #[serde(rename = "IO")]
    Ios,
}
