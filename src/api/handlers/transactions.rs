use axum::{extract::{Path, Query, State, Extension}, Json};
use deadpool_postgres::Pool;
use uuid::Uuid;
use crate::{
    db::{dal::{accounts as account_queries, transactions as transaction_queries}},
    base::{
        models::{transactions::{Transaction, CreateTransactionRequest, UpdateTransactionStatusRequest, TransactionPaginationParams}},
        error::AppError,
    },
    api::middleware::auth::AuthUser,
};

pub async fn create_transaction(
    Extension(auth): Extension<AuthUser>,
    State(pool): State<Pool>,
    Json(transaction): Json<CreateTransactionRequest>,
) -> Result<Json<Transaction>, AppError> {
    let client = pool.get().await.map_err(|e| AppError::Database(e.to_string()))?;
    
    // Verifying account ownership
    let account = account_queries::get_account_by_id(&client, transaction.account_id)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Account not found".into()))?;

    if account.user_id != auth.user_id {
        return Err(AppError::Auth("Unauthorized to create transaction for this account".into()));
    }

    let transaction = transaction_queries::create_transaction(&client, &transaction)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
    
    Ok(Json(transaction))
}

pub async fn get_transaction(
    Extension(auth): Extension<AuthUser>,
    State(pool): State<Pool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Transaction>, AppError> {
    let client = pool.get().await.map_err(|e| AppError::Database(e.to_string()))?;
    
    let transaction = transaction_queries::get_transaction_by_id(&client, id)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Transaction not found".into()))?;
    
    // Verifying account ownership
    let account = account_queries::get_account_by_id(&client, transaction.account_id)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Account not found".into()))?;
    
    if account.user_id != auth.user_id {
        return Err(AppError::Auth("Unauthorized to view this transaction".into()));
    }
    
    Ok(Json(transaction))
}

pub async fn update_transaction_status(
    Extension(auth): Extension<AuthUser>,
    State(pool): State<Pool>,
    Path(id): Path<Uuid>,
    Json(status): Json<UpdateTransactionStatusRequest>,
) -> Result<Json<Transaction>, AppError> {
    let client = pool.get().await.map_err(|e| AppError::Database(e.to_string()))?;
    
    let transaction = transaction_queries::get_transaction_by_id(&client, id)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Transaction not found".into()))?;
    
    // Verifying account ownership
    let account = account_queries::get_account_by_id(&client, transaction.account_id)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Account not found".into()))?;
    
    if account.user_id != auth.user_id {
        return Err(AppError::Auth("Unauthorized to update this transaction".into()));
    }
    
    let updated = transaction_queries::update_transaction_status(&client, id, &status)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Transaction not found".into()))?;
    
    Ok(Json(updated))
}

pub async fn list_transactions(
    Extension(auth): Extension<AuthUser>,
    State(pool): State<Pool>,
    Query(params): Query<TransactionPaginationParams>,
) -> Result<Json<Vec<Transaction>>, AppError> {
    let client = pool.get().await.map_err(|e| AppError::Database(e.to_string()))?;

    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    // If account_id is provided, verify ownership
    if let Some(account_id) = params.account_id {
        let account = account_queries::get_account_by_id(&client, account_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Account not found".into()))?;

        if account.user_id != auth.user_id {
            return Err(AppError::Auth("Unauthorized to view transactions for this account".into()));
        }
    }

    let transactions = transaction_queries::list_filtered_transactions(
        &client,
        auth.user_id,
        params.account_id,
        params.transaction_type.as_ref(),
        params.status.as_ref(),
        offset,
        per_page
    )
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(Json(transactions))
}