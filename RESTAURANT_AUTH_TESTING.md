# Restaurant Authentication Testing Guide

This guide provides curl commands to test the restaurant authentication endpoints.

## 🏪 Restaurant Endpoints

### 1. Create Restaurant (Public)
```bash
curl -X POST http://127.0.0.1:8001/restaurant \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Pizza Palace",
    "email": "admin@pizzapalace.com",
    "password": "RestaurantPass123",
    "contact_number": "+1234567890",
    "address": "123 Food Street, City, State",
    "cuisines": [
      {
        "name": "Italian",
        "description": "Authentic Italian cuisine"
      },
      {
        "name": "Pizza"
      }
    ]
  }'
```

**Expected Response:**
```json
{
  "Success": true,
  "message": "Restaurant created successfully",
  "restaurant": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "Pizza Palace",
    "email": "admin@pizzapalace.com",
    "contact number": "+1234567890",
    "address": "123 Food Street, City, State"
  }
}
```

### 2. Restaurant Login (Get JWT Token)
```bash
curl -X POST http://127.0.0.1:8001/restaurant/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "admin@pizzapalace.com",
    "password": "RestaurantPass123"
  }'
```

**Expected Response:**
```json
{
  "Success": true,
  "message": "Login successful",
  "restaurant_id": "550e8400-e29b-41d4-a716-446655440000",
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..."
}
```

### 3. Check Restaurant Authentication Status ✨ **NEW ENDPOINT**
```bash
# Replace YOUR_JWT_TOKEN with the actual token from login response
curl -X GET http://127.0.0.1:8001/restaurant/auth-check \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

**Expected Response (Authenticated):**
```json
{
  "Success": true,
  "message": "Restaurant is authenticated",
  "restaurant": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "Pizza Palace",
    "email": "admin@pizzapalace.com",
    "contact_number": "+1234567890",
    "address": "123 Food Street, City, State",
    "created_at": "2024-12-26T10:30:00Z",
    "updated_at": "2024-12-26T10:30:00Z"
  }
}
```

## 🔒 Authentication Error Tests

### 1. Missing Authorization Header
```bash
curl -X GET http://127.0.0.1:8001/restaurant/auth-check
```

**Expected Response (401 Unauthorized):**
```json
{
  "Success": false,
  "message": "Authorization header missing"
}
```

### 2. Invalid Token Format
```bash
curl -X GET http://127.0.0.1:8001/restaurant/auth-check \
  -H "Authorization: InvalidToken"
```

**Expected Response (401 Unauthorized):**
```json
{
  "Success": false,
  "message": "Authorization header must start with 'Bearer '"
}
```

### 3. Expired/Invalid Token
```bash
curl -X GET http://127.0.0.1:8001/restaurant/auth-check \
  -H "Authorization: Bearer invalid.jwt.token"
```

**Expected Response (401 Unauthorized):**
```json
{
  "Success": false,
  "message": "Invalid or expired token"
}
```

### 4. Valid Token but Restaurant Not Found
```bash
# This would happen if restaurant was deleted but token is still valid
curl -X GET http://127.0.0.1:8001/restaurant/auth-check \
  -H "Authorization: Bearer VALID_TOKEN_BUT_RESTAURANT_DELETED"
```

**Expected Response (401 Unauthorized):**
```json
{
  "Success": false,
  "message": "Restaurant not found"
}
```

## 🔄 Complete Restaurant Auth Workflow

### Step-by-Step Test Script
```bash
#!/bin/bash

echo "=== Restaurant Authentication Test ==="

# 1. Create a restaurant
echo "1. Creating restaurant..."
RESTAURANT_RESPONSE=$(curl -s -X POST http://127.0.0.1:8001/restaurant \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Test Restaurant",
    "email": "test@restaurant.com",
    "password": "TestPass123",
    "contact_number": "+1111111111",
    "address": "123 Test Street, Test City",
    "cuisines": [
      {
        "name": "Test Cuisine",
        "description": "Test description"
      }
    ]
  }')
echo "Restaurant creation: $RESTAURANT_RESPONSE"

# 2. Login to get token
echo -e "\n2. Logging in..."
LOGIN_RESPONSE=$(curl -s -X POST http://127.0.0.1:8001/restaurant/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@restaurant.com",
    "password": "TestPass123"
  }')
echo "Login response: $LOGIN_RESPONSE"

# Extract token (requires jq)
TOKEN=$(echo $LOGIN_RESPONSE | jq -r '.token')
echo "Extracted token: $TOKEN"

# 3. Check authentication status
echo -e "\n3. Checking authentication status..."
AUTH_CHECK_RESPONSE=$(curl -s -X GET http://127.0.0.1:8001/restaurant/auth-check \
  -H "Authorization: Bearer $TOKEN")
echo "Auth check response: $AUTH_CHECK_RESPONSE"

# 4. Test without token (should fail)
echo -e "\n4. Testing without token (should fail)..."
NO_TOKEN_RESPONSE=$(curl -s -X GET http://127.0.0.1:8001/restaurant/auth-check)
echo "No token test: $NO_TOKEN_RESPONSE"

# 5. Test with invalid token (should fail)
echo -e "\n5. Testing with invalid token (should fail)..."
INVALID_TOKEN_RESPONSE=$(curl -s -X GET http://127.0.0.1:8001/restaurant/auth-check \
  -H "Authorization: Bearer invalid.token.here")
echo "Invalid token test: $INVALID_TOKEN_RESPONSE"

echo -e "\n=== Restaurant Auth Test Complete ==="
```

Save as `test_restaurant_auth.sh`, make executable with `chmod +x test_restaurant_auth.sh`, and run with `./test_restaurant_auth.sh`

## 🎯 Use Cases for Restaurant Auth Check

This endpoint is useful for:

1. **Frontend Authentication**: Check if restaurant is still logged in before showing admin panels
2. **Session Validation**: Verify token validity before allowing restaurant operations
3. **User Info Retrieval**: Get current restaurant details for display
4. **Security Checks**: Ensure restaurant exists and token is valid before sensitive operations
5. **Dashboard Loading**: Fetch restaurant info to populate admin dashboard

## 📊 Response Status Codes

- **200 OK**: Restaurant is authenticated and found
- **401 Unauthorized**: Missing, invalid, or expired token / Restaurant not found
- **500 Internal Server Error**: Database or server error

## 🔗 Integration with Other Endpoints

Use this endpoint before calling other restaurant-specific endpoints that require authentication, such as:
- Menu management
- Order processing
- Restaurant profile updates
- Analytics and reports
