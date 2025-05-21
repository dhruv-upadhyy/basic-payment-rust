use crate::base::models::accounts::{Account, CreateAccountRequest, UpdateAccountRequest};
use deadpool_postgres::Client;
use tokio_postgres::Error;
use uuid::Uuid;
use rust_decimal::Decimal;

pub async fn create_account(client: &Client, account: &CreateAccountRequest) -> Result<Account, Error> {
    let currency: String = account.currency.clone().unwrap_or("INR".to_string());
    let initial_balance: Decimal = account.initial_balance.unwrap_or(Decimal::from(0));

    let statement = client
        .prepare(
            "INSERT INTO accounts (user_id, balance, currency) VALUES ($1, $2, $3)
             RETURNING id, user_id, balance, currency, created_at, updated_at",
        )
        .await?;

    client
        .query_one(&statement, &[&account.user_id, &initial_balance, &currency])
        .await?
        .try_into()
}

pub async fn get_account_by_id(client: &Client, id: Uuid) -> Result<Option<Account>, Error> {
    let statement = client
        .prepare(
            "SELECT id, user_id, balance, currency, created_at, updated_at
             FROM accounts WHERE id = $1",
        )
        .await?;

    Ok(client
        .query_opt(&statement, &[&id])
        .await?
        .map(|row| row.try_into().unwrap()))
}

pub async fn get_accounts_by_user_id(
    client: &Client,
    user_id: Uuid,
    offset: i64,
    limit: i64,
) -> Result<Vec<Account>, Error> {
    let statement = client
        .prepare(
            "SELECT id, user_id, balance, currency, created_at, updated_at
             FROM accounts
             WHERE user_id = $1
             ORDER BY created_at DESC
             LIMIT $2 OFFSET $3",
        )
        .await?;

    let rows = client.query(&statement, &[&user_id, &limit, &offset]).await?;
    Ok(rows.into_iter().map(|row| row.try_into().unwrap()).collect())
}

pub async fn update_account(
    client: &Client,
    id: Uuid,
    account: &UpdateAccountRequest,
) -> Result<Option<Account>, Error> {
    let statement = client
        .prepare(
            "UPDATE accounts
             SET currency = COALESCE($1, currency),
             updated_at = NOW()
             WHERE id = $2
             RETURNING id, user_id, balance, currency, created_at, updated_at",
        )
        .await?;

    Ok(client
        .query_opt(
            &statement,
            &[&account.currency, &id],
        )
        .await?
        .map(|row| row.try_into().unwrap()))
}

pub async fn delete_account(client: &Client, id: Uuid) -> Result<bool, Error> {
    let statement = client
        .prepare("DELETE FROM accounts WHERE id = $1")
        .await?;

    Ok(client.execute(&statement, &[&id]).await? > 0)
}

pub async fn update_account_balance(
    client: &Client,
    id: Uuid,
    amount: Decimal,
    is_credit: bool,
) -> Result<Option<Account>, Error> {
    let statement = if is_credit {
        client
            .prepare(
                "UPDATE accounts
                 SET balance = balance + $1,
                 updated_at = NOW()
                 WHERE id = $2
                 RETURNING id, user_id, balance, currency, created_at, updated_at",
            )
            .await?
    } else {
        client
            .prepare(
                "UPDATE accounts
                 SET balance = balance - $1,
                 updated_at = NOW()
                 WHERE id = $2 AND balance >= $1
                 RETURNING id, user_id, balance, currency, created_at, updated_at",
            )
            .await?
    };

    Ok(client
        .query_opt(
            &statement,
            &[&amount, &id],
        )
        .await?
        .map(|row| row.try_into().unwrap()))
}
