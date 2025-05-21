use crate::base::models::transactions::{Transaction, CreateTransactionRequest, TransactionStatus, TransactionType, UpdateTransactionStatusRequest};
use deadpool_postgres::Client;
use tokio_postgres::Error;
use uuid::Uuid;

pub async fn create_transaction(client: &Client, transaction: &CreateTransactionRequest) -> Result<Transaction, Error> {
    let description = transaction.description.clone().unwrap_or_default();
    let transaction_type = transaction.transaction_type.to_string();
    let status = TransactionStatus::Pending.to_string();

    let statement = client
        .prepare(
            "INSERT INTO transactions (account_id, amount, type, status, description) 
             VALUES ($1, $2, $3, $4, $5) 
             RETURNING id, account_id, amount, type, status, description, created_at, updated_at",
        )
        .await?;

    client
        .query_one(&statement, &[&transaction.account_id, &transaction.amount, &transaction_type, &status, &description])
        .await?
        .try_into()
}

pub async fn get_transaction_by_id(client: &Client, id: Uuid) -> Result<Option<Transaction>, Error> {
    let statement = client
        .prepare(
            "SELECT id, account_id, amount, type, status, description, created_at, updated_at 
             FROM transactions WHERE id = $1",
        )
        .await?;

    Ok(client
        .query_opt(&statement, &[&id])
        .await?
        .map(|row| row.try_into().unwrap()))
}

pub async fn update_transaction_status(
    client: &Client,
    id: Uuid,
    status_request: &UpdateTransactionStatusRequest,
) -> Result<Option<Transaction>, Error> {
    let status = status_request.status.to_string();

    let statement = client
        .prepare(
            "UPDATE transactions 
             SET status = $1,
             updated_at = NOW()
             WHERE id = $2
             RETURNING id, account_id, amount, type, status, description, created_at, updated_at",
        )
        .await?;

    Ok(client
        .query_opt(
            &statement,
            &[&status, &id],
        )
        .await?
        .map(|row| row.try_into().unwrap()))
}

pub async fn list_filtered_transactions(
    client: &Client,
    user_id: Uuid,
    account_id: Option<Uuid>,
    transaction_type: Option<&TransactionType>,
    status: Option<&TransactionStatus>,
    offset: i64,
    limit: i64,
) -> Result<Vec<Transaction>, Error> {
    let mut query = String::from(
        "SELECT t.id, t.account_id, t.amount, t.type, t.status, t.description, t.created_at, t.updated_at 
         FROM transactions t
         JOIN accounts a ON t.account_id = a.id
         WHERE a.user_id = $1"
    );

    let mut params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = vec![&user_id];

    let mut type_str_owned = None;
    let mut status_str_owned = None;

    if let Some(acc_id) = &account_id {
        query.push_str(&format!(" AND t.account_id = ${}", params.len() + 1));
        params.push(acc_id);
    }

    if let Some(t_type) = transaction_type {
        type_str_owned = Some(t_type.to_string());
        query.push_str(&format!(" AND t.type = ${}", params.len() + 1));
        params.push(type_str_owned.as_ref().unwrap());
    }

    if let Some(t_status) = status {
        status_str_owned = Some(t_status.to_string());
        query.push_str(&format!(" AND t.status = ${}", params.len() + 1));
        params.push(status_str_owned.as_ref().unwrap());
    }

    query.push_str(&format!(
        " ORDER BY t.created_at DESC LIMIT ${} OFFSET ${}",
        params.len() + 1,
        params.len() + 2
    ));
    params.push(&limit);
    params.push(&offset);

    let statement = client.prepare(&query).await?;
    let rows = client.query(&statement, &params[..]).await?;

    Ok(rows.into_iter().map(|row| row.try_into().unwrap()).collect())
}
