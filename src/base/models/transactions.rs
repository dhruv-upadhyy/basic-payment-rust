use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use std::convert::TryFrom;
use uuid::Uuid;
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransactionType {
    #[serde(rename = "WITHDRAWAL")]
    Withdrawal,
    #[serde(rename = "DEPOSIT")]
    Deposit,
}

impl ToString for TransactionType {
    fn to_string(&self) -> String {
        match self {
            TransactionType::Withdrawal => "WITHDRAWAL".to_string(),
            TransactionType::Deposit => "DEPOSIT".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransactionStatus {
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "COMPLETED")]
    Completed,
    #[serde(rename = "FAILED")]
    Failed,
}

impl ToString for TransactionStatus {
    fn to_string(&self) -> String {
        match self {
            TransactionStatus::Pending => "PENDING".to_string(),
            TransactionStatus::Completed => "COMPLETED".to_string(),
            TransactionStatus::Failed => "FAILED".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub account_id: Uuid,
    pub amount: Decimal,
    pub transaction_type: TransactionType,
    pub status: TransactionStatus,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TryFrom<Row> for Transaction {
    type Error = tokio_postgres::Error;

    fn try_from(row: Row) -> Result<Self, Self::Error> {
        let transaction_type_str: String = row.get("type");
        let transaction_type = match transaction_type_str.as_str() {
            "WITHDRAWAL" => TransactionType::Withdrawal,
            "DEPOSIT" => TransactionType::Deposit,
            _ => TransactionType::Withdrawal,
        };

        let status_str: String = row.get("status");
        let status = match status_str.as_str() {
            "PENDING" => TransactionStatus::Pending,
            "COMPLETED" => TransactionStatus::Completed,
            "FAILED" => TransactionStatus::Failed,
            _ => TransactionStatus::Pending,
        };

        Ok(Transaction {
            id: row.get("id"),
            account_id: row.get("account_id"),
            amount: row.get("amount"),
            transaction_type,
            status,
            description: row.get("description"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateTransactionRequest {
    pub account_id: Uuid,
    pub amount: Decimal,
    pub transaction_type: TransactionType,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTransactionStatusRequest {
    pub status: TransactionStatus,
}

#[derive(Debug, Deserialize)]
pub struct TransactionPaginationParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub account_id: Option<Uuid>,
    pub transaction_type: Option<TransactionType>,
    pub status: Option<TransactionStatus>,
} 