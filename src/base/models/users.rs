use std::convert::TryFrom;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::base::utils::ts_rfc3339;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    #[serde(with = "ts_rfc3339")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_rfc3339")]
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing)]
    pub password: String,
}

impl TryFrom<Row> for User {
    type Error = tokio_postgres::Error;

    fn try_from(row: Row) -> Result<Self, Self::Error> {
        Ok(User {
            id: row.get("id"),
            name: row.get("name"),
            email: row.get("email"),
            password: row.try_get("password").unwrap_or("".to_string()),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: User,
}

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}