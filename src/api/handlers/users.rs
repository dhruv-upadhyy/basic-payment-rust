use axum::{extract::{Path, Query, State, Extension}, Json};
use deadpool_postgres::Pool;
use uuid::Uuid;
use crate::{
    base::{
        error::AppError,
        utils::{hash_password, validate_password},
        models::users::{CreateUserRequest, LoginRequest, LoginResponse, PaginationParams, UpdateUserRequest, User},
    },
    db::dal::users as user_queries,
    api::middleware::auth::{create_token, AuthUser},
};

pub async fn create_user(
    State(pool): State<Pool>,
    Json(mut user): Json<CreateUserRequest>,
) -> Result<Json<User>, AppError> {
    let db_client = pool.get().await.map_err(|e| AppError::Database(e.to_string()))?;

    let password_hash = hash_password(&user.password).map_err(|e| AppError::Auth(e.to_string()))?;
    user.password = password_hash;

    let user = user_queries::create_user(&db_client, &user)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(Json(user))
}

pub async fn login(
    State(pool): State<Pool>,
    Json(credentials): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let db_client = pool.get().await.map_err(|e| AppError::Database(e.to_string()))?;

    let user = user_queries::get_user_by_email(&db_client, &credentials.email)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::Auth("Invalid credentials".into()))?;

    if validate_password(&credentials.password, &user.password).is_err() {
        return Err(AppError::Auth("Invalid password".into()));
    }

    let token = create_token(user.id)?;

    Ok(Json(LoginResponse { token, user }))
}

pub async fn get_user(
    Extension(_auth): Extension<AuthUser>,
    State(pool): State<Pool>,
    Path(id): Path<Uuid>,
) -> Result<Json<User>, AppError> {
    let client = pool.get().await.map_err(|e| AppError::Database(e.to_string()))?;

    let user = user_queries::get_user_by_id(&client, id)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    Ok(Json(user))
}

pub async fn update_user(
    Extension(auth): Extension<AuthUser>,
    State(pool): State<Pool>,
    Path(id): Path<Uuid>,
    Json(mut user): Json<UpdateUserRequest>,
) -> Result<Json<User>, AppError> {
    if auth.user_id != id {
        return Err(AppError::Auth("Unauthorized".into()));
    }

    let db_client = pool.get().await.map_err(|e| AppError::Database(e.to_string()))?;

    if let Some(ref password) = user.password {
        let password_hash = hash_password(password).map_err(|e| AppError::Auth(e.to_string()))?;
        user.password = Some(password_hash);
    }

    let user = user_queries::update_user(&db_client, id, &user)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    Ok(Json(user))
}

pub async fn delete_user(
    Extension(auth): Extension<AuthUser>,
    State(pool): State<Pool>,
    Path(id): Path<Uuid>,
) -> Result<(), AppError> {
    if auth.user_id != id {
        return Err(AppError::Auth("Unauthorized".into()));
    }

    let db_client = pool.get().await.map_err(|e| AppError::Database(e.to_string()))?;
    
    let deleted = user_queries::delete_user(&db_client, id)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
    
    if !deleted {
        return Err(AppError::NotFound("User not found".into()));
    }
    
    Ok(())
}

pub async fn list_users(
    Extension(_auth): Extension<AuthUser>,
    State(pool): State<Pool>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<Vec<User>>, AppError> {
    let db_client = pool.get().await.map_err(|e| AppError::Database(e.to_string()))?;
    
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;
    
    let users = user_queries::list_users(&db_client, offset, per_page)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
    
    Ok(Json(users))
}
