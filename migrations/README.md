# Database Migrations

This directory contains SQL migration files for the auth service database.

## Migration Files

1. **20241226000001_create_database.sql** - Sets up the database configuration
2. **20241226000002_create_customers_table.sql** - Creates the customers table
3. **20241226000003_create_addresses_table.sql** - Creates the addresses table
4. **20241226000004_create_updated_at_triggers.sql** - Creates triggers for automatic timestamp updates
5. **20241226000005_rollback_migrations.sql** - Rollback script to drop all objects

## Running Migrations

### Using SQLx CLI

1. Install sqlx-cli if not already installed:
   ```bash
   cargo install sqlx-cli
   ```

2. Run migrations:
   ```bash
   sqlx migrate run
   ```

3. To revert migrations:
   ```bash
   sqlx migrate revert
   ```

### Manual Execution

1. Start your CockroachDB instance:
   ```bash
   docker-compose up -d
   ```

2. Connect to the database:
   ```bash
   cockroach sql --insecure --host=localhost:26257
   ```

3. Execute migrations in order:
   ```sql
   \i migrations/20241226000001_create_database.sql
   \i migrations/20241226000002_create_customers_table.sql
   \i migrations/20241226000003_create_addresses_table.sql
   \i migrations/20241226000004_create_updated_at_triggers.sql
   ```

## Database Schema

### Customers Table
- `id` (UUID, Primary Key)
- `name` (VARCHAR(255), NOT NULL)
- `email` (VARCHAR(255), NOT NULL, UNIQUE)
- `password` (VARCHAR(255), NOT NULL)
- `phone` (VARCHAR(20), NOT NULL)
- `created_at` (TIMESTAMPTZ, DEFAULT NOW())
- `updated_at` (TIMESTAMPTZ, DEFAULT NOW())

### Addresses Table
- `id` (UUID, Primary Key)
- `customer_id` (UUID, Foreign Key to customers.id)
- `name` (VARCHAR(255), NOT NULL)
- `pincode` (VARCHAR(10), NOT NULL)
- `phone` (VARCHAR(20), NOT NULL)
- `address` (TEXT, NOT NULL)
- `is_default` (BOOLEAN, DEFAULT FALSE)
- `created_at` (TIMESTAMPTZ, DEFAULT NOW())
- `updated_at` (TIMESTAMPTZ, DEFAULT NOW())

## Features

- Automatic UUID generation for primary keys
- Automatic timestamp updates via triggers
- Foreign key constraints with cascade delete
- Unique constraints for email and default addresses
- Indexes for performance optimization
- Comprehensive comments for documentation
