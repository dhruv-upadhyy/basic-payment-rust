use axum::{extract::{Path, Query, State, Extension}, Json};
use deadpool_postgres::Pool;
use uuid::Uuid;
use rust_decimal::Decimal;
use crate::{
    db::dal::{accounts as account_queries, transactions as transaction_queries},
    base::{
        models::{
            accounts::{Account, CreateAccountRequest, UpdateAccountRequest, AccountPaginationParams, DepositRequest, WithdrawalRequest},
            transactions::{CreateTransactionRequest, TransactionType},
        },
        error::AppError,
    },
    api::middleware::auth::AuthUser,
};

pub async fn create_account(
    Extension(auth): Extension<AuthUser>,
    State(pool): State<Pool>,
    Json(mut account): Json<CreateAccountRequest>,
) -> Result<Json<Account>, AppError> {
    let client = pool.get().await.map_err(|e| AppError::Database(e.to_string()))?;

    // Only authenticated users can create an account
    account.user_id = auth.user_id;

    let account: Account = account_queries::create_account(&client, &account)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(Json(account))
}

pub async fn get_account(
    Extension(auth): Extension<AuthUser>,
    State(pool): State<Pool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Account>, AppError> {
    let client = pool.get().await.map_err(|e| AppError::Database(e.to_string()))?;

    let account = account_queries::get_account_by_id(&client, id)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Account not found".into()))?;

    // User can only view their own accounts
    if account.user_id != auth.user_id {
        return Err(AppError::Auth("Unauthorized access to account".into()));
    }

    Ok(Json(account))
}

pub async fn update_account(
    Extension(auth): Extension<AuthUser>,
    State(pool): State<Pool>,
    Path(id): Path<Uuid>,
    Json(account): Json<UpdateAccountRequest>,
) -> Result<Json<Account>, AppError> {
    let client = pool.get().await.map_err(|e| AppError::Database(e.to_string()))?;

    // Verifying account ownership
    let existing = account_queries::get_account_by_id(&client, id)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Account not found".into()))?;

    if existing.user_id != auth.user_id {
        return Err(AppError::Auth("Unauthorized to update this account".into()));
    }

    let updated_account = account_queries::update_account(&client, id, &account)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Account not found".into()))?;

    Ok(Json(updated_account))
}

pub async fn delete_account(
    Extension(auth): Extension<AuthUser>,
    State(pool): State<Pool>,
    Path(id): Path<Uuid>,
) -> Result<(), AppError> {
    let client = pool.get().await.map_err(|e| AppError::Database(e.to_string()))?;

    // Verify account ownership
    let existing = account_queries::get_account_by_id(&client, id)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Account not found".into()))?;

    if existing.user_id != auth.user_id {
        return Err(AppError::Auth("Unauthorized to delete this account".into()));
    }

    let deleted = account_queries::delete_account(&client, id)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    if !deleted {
        return Err(AppError::NotFound("Account not found".into()));
    }

    Ok(())
}

pub async fn list_accounts(
    Extension(auth): Extension<AuthUser>,
    State(pool): State<Pool>,
    Query(params): Query<AccountPaginationParams>,
) -> Result<Json<Vec<Account>>, AppError> {
    let client = pool.get().await.map_err(|e| AppError::Database(e.to_string()))?;

    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    // If user_id is provided and doesn't match authenticated user, return error
    if let Some(user_id) = params.user_id {
        if user_id != auth.user_id {
            return Err(AppError::Auth("Unauthorized to view other users' accounts".into()));
        }
    }

    let accounts = account_queries::get_accounts_by_user_id(&client, auth.user_id, offset, per_page)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(Json(accounts))
}

pub async fn deposit(
    Extension(auth): Extension<AuthUser>,
    State(pool): State<Pool>,
    Path(id): Path<Uuid>,
    Json(deposit): Json<DepositRequest>,
) -> Result<Json<Account>, AppError> {
    let client = pool.get().await.map_err(|e| AppError::Database(e.to_string()))?;

    if deposit.amount <= Decimal::from(0) {
        return Err(AppError::Validation("Invalid amount".into()));
    }

    // Verifying account ownership
    let account = account_queries::get_account_by_id(&client, id)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Account not found".into()))?;

    if account.user_id != auth.user_id {
        return Err(AppError::Auth("Unauthorized to deposit to this account".into()));
    }

    // Creating transaction
    let transaction_request = CreateTransactionRequest {
        account_id: id,
        amount: deposit.amount,
        transaction_type: TransactionType::Deposit,
        description: deposit.description.clone(),
    };

    let _transaction = transaction_queries::create_transaction(&client, &transaction_request)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    // Adding amount to account balance
    let updated_account = account_queries::update_account_balance(&client, id, deposit.amount, true)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Account not found or balance update failed".into()))?;

    Ok(Json(updated_account))
}

pub async fn withdraw(
    Extension(auth): Extension<AuthUser>,
    State(pool): State<Pool>,
    Path(id): Path<Uuid>,
    Json(withdrawal): Json<WithdrawalRequest>,
) -> Result<Json<Account>, AppError> {
    let client = pool.get().await.map_err(|e| AppError::Database(e.to_string()))?;

    if withdrawal.amount <= Decimal::from(0) {
        return Err(AppError::Validation("Invalid amount".into()));
    }

    // Verifying account ownership
    let account = account_queries::get_account_by_id(&client, id)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Account not found".into()))?;

    if account.user_id != auth.user_id {
        return Err(AppError::Auth("Unauthorized to withdraw from this account".into()));
    }

    if account.balance < withdrawal.amount {
        return Err(AppError::Validation("Insufficient balance".into()));
    }

    // Creating transaction
    let transaction_request = CreateTransactionRequest {
        account_id: id,
        amount: withdrawal.amount,
        transaction_type: TransactionType::Withdrawal,
        description: withdrawal.description.clone(),
    };

    let _transaction = transaction_queries::create_transaction(&client, &transaction_request)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    // Subtracting amount from account balance
    let updated_account = account_queries::update_account_balance(&client, id, withdrawal.amount, false)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::Validation("Insufficient balance or account not found".into()))?;

    Ok(Json(updated_account))
}
