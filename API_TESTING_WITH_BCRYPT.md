# API Testing Guide with Bcrypt Password Hashing

This guide provides curl commands to test the auth service API with bcrypt password hashing and validation.

## 🔐 Password Requirements

The API now enforces the following password requirements:
- **Minimum 8 characters**
- **At least one uppercase letter (A-Z)**
- **At least one lowercase letter (a-z)**
- **At least one number (0-9)**

## 📧 Email Requirements

- Must contain '@' and '.' characters
- Minimum 6 characters total
- Must be unique (no duplicates allowed)

## 🚀 API Endpoints

### 1. Health Check
```bash
curl -X GET http://127.0.0.1:8001/health
```

### 2. Create Customer (Valid Request)
```bash
curl -X POST http://127.0.0.1:8001/customer \
  -H "Content-Type: application/json" \
  -d '{
    "name": "John Doe",
    "email": "john.doe@example.com",
    "password": "SecurePass123",
    "phone": "+1234567890"
  }'
```

**Expected Response:**
```json
{
  "Success": true,
  "message": "Customer created successfully",
  "customer": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "John Doe",
    "email": "john.doe@example.com",
    "phone": "+1234567890"
  }
}
```

### 3. Test Password Validation Errors

#### Password Too Short
```bash
curl -X POST http://127.0.0.1:8001/customer \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Test User",
    "email": "test1@example.com",
    "password": "short",
    "phone": "+1111111111"
  }'
```

**Expected Response (400 Bad Request):**
```json
{
  "Success": false,
  "message": "Password must be at least 8 characters long"
}
```

#### Missing Uppercase Letter
```bash
curl -X POST http://127.0.0.1:8001/customer \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Test User",
    "email": "test2@example.com",
    "password": "lowercase123",
    "phone": "+2222222222"
  }'
```

**Expected Response (400 Bad Request):**
```json
{
  "Success": false,
  "message": "Password must contain at least one uppercase letter"
}
```

#### Missing Lowercase Letter
```bash
curl -X POST http://127.0.0.1:8001/customer \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Test User",
    "email": "test3@example.com",
    "password": "UPPERCASE123",
    "phone": "+3333333333"
  }'
```

#### Missing Number
```bash
curl -X POST http://127.0.0.1:8001/customer \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Test User",
    "email": "test4@example.com",
    "password": "NoNumbers",
    "phone": "+4444444444"
  }'
```

### 4. Test Email Validation

#### Invalid Email Format
```bash
curl -X POST http://127.0.0.1:8001/customer \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Test User",
    "email": "invalid-email",
    "password": "ValidPass123",
    "phone": "+5555555555"
  }'
```

**Expected Response (400 Bad Request):**
```json
{
  "Success": false,
  "message": "Invalid email format"
}
```

### 5. Test Duplicate Email

#### First Customer
```bash
curl -X POST http://127.0.0.1:8001/customer \
  -H "Content-Type: application/json" \
  -d '{
    "name": "First User",
    "email": "duplicate@example.com",
    "password": "FirstPass123",
    "phone": "+6666666666"
  }'
```

#### Duplicate Email Attempt
```bash
curl -X POST http://127.0.0.1:8001/customer \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Second User",
    "email": "duplicate@example.com",
    "password": "SecondPass123",
    "phone": "+7777777777"
  }'
```

**Expected Response (409 Conflict):**
```json
{
  "Success": false,
  "message": "Email address already exists"
}
```

## 🔍 Verify Password Hashing

To verify that passwords are properly hashed in the database:

```bash
docker exec cockroachdb ./cockroach sql --insecure --database=auth_service --execute="SELECT email, password FROM customers LIMIT 5;"
```

You should see hashed passwords that look like:
```
$2b$12$LQv3c1yqBWVHxkd0LQ4YCOWpuqhyxc8aibQuvFWqIjpwxne4BVh4S
```

## 🚀 Quick Test Script

Save this as `test_api.sh`:
```bash
#!/bin/bash

echo "Testing API with bcrypt password hashing..."

# Test health check
echo "1. Health check:"
curl -s http://127.0.0.1:8001/health | jq

# Test valid customer creation
echo -e "\n2. Valid customer creation:"
curl -s -X POST http://127.0.0.1:8001/customer \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Jane Smith",
    "email": "jane.smith@test.com",
    "password": "StrongPass123",
    "phone": "+1234567890"
  }' | jq

# Test password validation
echo -e "\n3. Weak password test:"
curl -s -X POST http://127.0.0.1:8001/customer \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Weak User",
    "email": "weak@test.com",
    "password": "weak",
    "phone": "+9876543210"
  }' | jq

echo -e "\nAPI testing completed!"
```

Make it executable: `chmod +x test_api.sh`
Run it: `./test_api.sh`
