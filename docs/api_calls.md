# Test API Calls

This document has all the cURL commands for testing the API endpoints.

## Setup

API server should be running at `http://localhost:3000`.

Set the following environment variables:
```bash
export API_URL=http://localhost:3000
export AUTH_TOKEN=your_jwt_token # Will be obtained after login
```

## User Endpoints

### Register new user

```bash
curl -X POST "$API_URL/users" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Test User",
    "email": "test@example.com",
    "password": "Password123!"
  }'
```

### Login

```bash
curl -X POST "$API_URL/users/login" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "Password123!"
  }'
```

Save the token from the response:
```bash
export AUTH_TOKEN=""  # Replace with the actual token
```

### Get User Details

```bash
curl -X GET "$API_URL/users/{user_id}" \
  -H "Authorization: Bearer $AUTH_TOKEN" 
```

### Update User

```bash
curl -X PUT "$API_URL/users/{user_id}" \
  -H "Authorization: Bearer $AUTH_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "New Username",
    "password": "NewPassword!"
  }'
```

### List Users

```bash
curl -X GET "$API_URL/users?page=1&per_page=10" \
  -H "Authorization: Bearer $AUTH_TOKEN"
```

### Delete User

```bash
curl -X DELETE "$API_URL/users/{user_id}" \
  -H "Authorization: Bearer $AUTH_TOKEN"
```

## Account Endpoints

### Create Account

```bash
curl -X POST "$API_URL/accounts" \
  -H "Authorization: Bearer $AUTH_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "currency": "INR",
    "initial_balance": 100.00
  }'
```

### List Accounts

```bash
curl -X GET "$API_URL/accounts?page=1&per_page=10" \
  -H "Authorization: Bearer $AUTH_TOKEN"
```

### Get Account Details

```bash
curl -X GET "$API_URL/accounts/{account_id}" \
  -H "Authorization: Bearer $AUTH_TOKEN"
```

### Update Account

```bash
curl -X PUT "$API_URL/accounts/{account_id}" \
  -H "Authorization: Bearer $AUTH_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "currency": "EUR"
  }'
```

### Delete Account

```bash
curl -X DELETE "$API_URL/accounts/{account_id}" \
  -H "Authorization: Bearer $AUTH_TOKEN"
```

### Deposit

```bash
curl -X POST "$API_URL/accounts/{account_id}/deposit" \
  -H "Authorization: Bearer $AUTH_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "amount": 50.00,
    "description": "Test deposit"
  }'
```

### Withdraw

```bash
curl -X POST "$API_URL/accounts/{account_id}/withdraw" \
  -H "Authorization: Bearer $AUTH_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "amount": 25.00,
    "description": "Test withdrawal"
  }'
```

## Transaction Endpoints

### Create Transaction

```bash
curl -X POST "$API_URL/transactions" \
  -H "Authorization: Bearer $AUTH_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "account_id": "account_id_here",
    "amount": 50.00,
    "transaction_type": "DEPOSIT",
    "description": "Test transaction"
  }'
```
Transaction Types: CREDIT, DEBIT

### List Transactions

```bash
curl -X GET "$API_URL/transactions?page=1&per_page=10&account_id=account_id" \
  -H "Authorization: Bearer $AUTH_TOKEN"
```
Additional filters along with account_id: transaction_type, status

### Get Transaction Details

```bash
curl -X GET "$API_URL/transactions/{transaction_id}" \
  -H "Authorization: Bearer $AUTH_TOKEN"
```

### Update Transaction Status

```bash
curl -X PUT "$API_URL/transactions/{transaction_id}/status" \
  -H "Authorization: Bearer $AUTH_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "status": "COMPLETED"
  }'
```
Status Types: PENDING, COMPLETED, FAILED
