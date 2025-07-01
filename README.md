# Rust based Payment System 

### Running with Docker

1. Create `.env` file with the following format:
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

2. Create tables provided in the SQL schema [`src/db/ddl.sql`](https://github.com/dhruv-upadhyy/dodo-assignment-rust/blob/main/src/db/ddl.sql) in `POSTGRES_DB` (configured in .env):

3. Run the application:
   ```bash
   cargo run --release
   ```

## API Documentation

OpenAPI documentation:  [`docs/openapi.yml`](https://github.com/dhruv-upadhyy/dodo-assignment-rust/blob/main/docs/openapi.yml).

All the cURL commands for testing the API endpoints:  [`docs/api_call.md`](https://github.com/dhruv-upadhyy/dodo-assignment-rust/blob/main/docs/api_calls.md).
