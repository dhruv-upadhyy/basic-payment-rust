use axum::{routing::{get, post, put, delete}, Router, middleware};
use deadpool_postgres::Pool;
use crate::api::{
    handlers::{users, accounts, transactions},
    middleware::{auth::auth_middleware, rate_limit::rate_limit_middleware}
};

pub fn create_router(pool: Pool) -> Router {
    let public_routes = Router::new()
        .route("/users", post(users::create_user))
        .route("/users/login", post(users::login));

    let protected_routes = Router::new()
        .route("/users", get(users::list_users))
        .route("/users/{id}", get(users::get_user))
        .route("/users/{id}", put(users::update_user))
        .route("/users/{id}", delete(users::delete_user))

        .route("/accounts", post(accounts::create_account))
        .route("/accounts", get(accounts::list_accounts))
        .route("/accounts/{id}", get(accounts::get_account))
        .route("/accounts/{id}", put(accounts::update_account))
        .route("/accounts/{id}", delete(accounts::delete_account))
        .route("/accounts/{id}/deposit", post(accounts::deposit))
        .route("/accounts/{id}/withdraw", post(accounts::withdraw))

        .route("/transactions", post(transactions::create_transaction))
        .route("/transactions", get(transactions::list_transactions))
        .route("/transactions/{id}", get(transactions::get_transaction))
        .route("/transactions/{id}/status", put(transactions::update_transaction_status))

        .layer(middleware::from_fn(auth_middleware));

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .layer(middleware::from_fn_with_state((), rate_limit_middleware))
        .with_state(pool)
}