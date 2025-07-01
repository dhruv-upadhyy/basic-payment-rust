# Basic Rust based Payment Backend

### Docker

1. Create `.env` file:
   ```bash
    # Database configuration
    POSTGRES_USER=test_user
    POSTGRES_PASSWORD=test_password
    POSTGRES_DB=test_db
    POSTGRES_PORT=5432

    #API configuration
    DATABASE_URL=postgresql://test_user:test_password@localhost:5432/test_db
    JWT_SECRET=secret
    JWT_EXPIRATION=24
    RUST_LOG=info
    API_PORT=3000
   ```

3. Start with Docker Compose:
   ```bash
   docker-compose up -d
   ```

4. The API will be available at `http://localhost:3000`

### Local

1. Use the same .env file format as above.

2. Create tables from the SQL schema: [`src/db/ddl.sql`](https://github.com/dhruv-upadhyy/basic-payment-rust/blob/main/src/db/ddl.sql) in `POSTGRES_DB` (configured in .env):

3. Run the application:
   ```bash
   cargo run --release
   ```

## API Documentation

OpenAPI spec: [`docs/openapi.yml`](https://github.com/dhruv-upadhyy/basic-payment-rust/blob/main/docs/openapi.yml).

Example cURL calls:  [`docs/api_call.md`](https://github.com/dhruv-upadhyy/basic-payment-rust/blob/main/docs/api_calls.md).
