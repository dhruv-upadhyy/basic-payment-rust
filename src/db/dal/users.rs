use crate::base::models::users::{CreateUserRequest, UpdateUserRequest, User};
use deadpool_postgres::Client;
use tokio_postgres::Error;
use uuid::Uuid;

pub async fn create_user(client: &Client, user: &CreateUserRequest) -> Result<User, Error> {
    let statement = client
        .prepare(
            "INSERT INTO users (name, email, password) 
             VALUES ($1, $2, $3) 
             RETURNING id, name, email, created_at, updated_at",
        )
        .await?;

    Ok(client
        .query_one(&statement, &[&user.name, &user.email, &user.password])
        .await?
        .try_into()
        .unwrap())
}

pub async fn get_user_by_id(client: &Client, id: Uuid) -> Result<Option<User>, Error> {
    let statement = client
        .prepare(
            "SELECT id, name, email, password, created_at, updated_at
             FROM users WHERE id = $1",
        )
        .await?;

    Ok(client
        .query_opt(&statement, &[&id])
        .await?
        .map(|row| row.try_into().unwrap()))
}

pub async fn get_user_by_email(client: &Client, email: &str) -> Result<Option<User>, Error> {
    let statement = client
        .prepare(
            "SELECT id, name, email, password, created_at, updated_at
             FROM users WHERE email = $1",
        )
        .await?;

    Ok(client
        .query_opt(&statement, &[&email])
        .await?
        .map(|row| row.try_into().unwrap()))
}

pub async fn update_user(
    client: &Client,
    id: Uuid,
    user: &UpdateUserRequest,
) -> Result<Option<User>, Error> {
    let statement = client
        .prepare(
            "UPDATE users 
             SET name = COALESCE($1, name), 
                 email = COALESCE($2, email), 
                 password = COALESCE($3, password),
                 updated_at = NOW()
             WHERE id = $4
             RETURNING id, name, email, created_at, updated_at",
        )
        .await?;

    Ok(client
        .query_opt(
            &statement,
            &[&user.name, &user.email, &user.password, &id],
        )
        .await?
        .map(|row| row.try_into().unwrap()))
}

pub async fn delete_user(client: &Client, id: Uuid) -> Result<bool, Error> {
    let statement = client
        .prepare("DELETE FROM users WHERE id = $1")
        .await?;

    Ok(client.execute(&statement, &[&id]).await? > 0)
}

pub async fn list_users(
    client: &Client,
    offset: i64,
    limit: i64,
) -> Result<Vec<User>, Error> {
    let statement = client
        .prepare(
            "SELECT id, name, email, created_at, updated_at
             FROM users 
             ORDER BY created_at DESC 
             LIMIT $1 OFFSET $2",
        )
        .await?;

    let rows = client.query(&statement, &[&limit, &offset]).await?;
    Ok(rows.into_iter().map(|row| row.try_into().unwrap()).collect())
} 