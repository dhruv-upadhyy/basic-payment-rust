use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use std::convert::TryFrom;
use uuid::Uuid;
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: Uuid,
    pub user_id: Uuid,
    pub balance: Decimal,
    pub currency: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TryFrom<Row> for Account {
    type Error = tokio_postgres::Error;

    fn try_from(row: Row) -> Result<Self, Self::Error> {
        Ok(Account {
            id: row.get("id"),
            user_id: row.get("user_id"),
            balance: row.get::<_, Decimal>("balance"),
            currency: row.get("currency"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateAccountRequest {
    pub user_id: Uuid,
    pub currency: Option<String>,
    pub initial_balance: Option<Decimal>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAccountRequest {
    pub currency: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DepositRequest {
    pub amount: Decimal,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct WithdrawalRequest {
    pub amount: Decimal,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AccountPaginationParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub user_id: Option<Uuid>,
}