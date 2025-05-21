# Dodo Payment Assignment

## Features

- **User Management**: Registration, authentication, and profile management
- **Account Management**: Create and manage multiple accounts
- **Transaction Processing**: Perform deposits and withdrawals, with transaction history
- **Security**: JWT-based authentication, rate limiting, data validation, and error handling

## Tech Stack

- **Axum**: Web framework
- **PostgreSQL**: Relational database
- **Deadpool**: PostgreSQL connection pooling
- **jsonwebtoken**: JWT authentication
- **Tower-http**: Middleware
- **Tracing**: Logging


## Setup

### Running with Docker

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd dodo-assignment-rust
   ```

2. Create `.env` file with the following format:
   ```bash
    # Database configuration
    POSTGRES_USER=test_user
    POSTGRES_PASSWORD=test_password
    POSTGRES_DB=test_db
    POSTGRES_PORT=5432

    #API configuration
    DATABASE_URL=postgresql://test_user:test_password@localhost:5432/test_db
    JWT_SECRET=dodo_assignment
    JWT_EXPIRATION=24
    RUST_LOG=info
    API_PORT=3000
   ```

3. Start the application using Docker Compose:
   ```bash
   docker-compose up -d
   ```

4. The API will be available at `http://localhost:3000`

### Running Locally

1. Create `.env` file with the following format:
   ```bash
    # Database configuration
    POSTGRES_USER=test_user
    POSTGRES_PASSWORD=test_password
    POSTGRES_DB=test_db
    POSTGRES_PORT=5432
    DATABASE_URL=postgresql://test_user:test_password@localhost:5432/test_db

    #API configuration
    DATABASE_URL=postgresql://test_user:test_password@localhost:5432/test_db
    JWT_SECRET=dodo_assignment
    JWT_EXPIRATION=24
    RUST_LOG=info
    API_PORT=3000
   ```

2. Create tables provided in the SQL schema [`src/db/ddl.sql`]([src/db/ddl.sql](https://github.com/dhruv-upadhyy/dodo-assignment-rust/blob/main/src/db/ddl.sql)) in POSTGRES_DB:

4. Build and run the application:
   ```bash
   cargo build
   cargo run
   ```

## API Documentation

OpenAPI documentation:  [`docs/openapi.yml`](https://github.com/dhruv-upadhyy/dodo-assignment-rust/blob/main/docs/openapi.yml).

All the cURL commands for testing the API endpoints:  [`docs/api_call.md`](https://github.com/dhruv-upadhyy/dodo-assignment-rust/blob/main/docs/api_calls.md).

## Authentication

Use `/users/login` endpoint to obtain a JWT token, then include it in subsequent requests:

```
Authorization: Bearer <token>
```

The token expires after 24 hours by default (configurable via JWT_EXPIRATION).

## Rate Limiting

By default, 100 requests are allowed per minute per IP address.

