-- Create customers table
CREATE TABLE IF NOT EXISTS customers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    phone VARCHAR(20) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Create index on email for faster lookups
CREATE INDEX IF NOT EXISTS idx_customers_email ON customers(email);

-- Create index on phone for faster lookups
CREATE INDEX IF NOT EXISTS idx_customers_phone ON customers(phone);

-- Add comments to the table and columns
COMMENT ON TABLE customers IS 'Stores customer information for the auth service';
COMMENT ON COLUMN customers.id IS 'Unique identifier for the customer';
COMMENT ON COLUMN customers.name IS 'Full name of the customer';
COMMENT ON COLUMN customers.email IS 'Email address of the customer (unique)';
COMMENT ON COLUMN customers.password IS 'Hashed password for authentication';
COMMENT ON COLUMN customers.phone IS 'Phone number of the customer';
COMMENT ON COLUMN customers.created_at IS 'Timestamp when the customer was created';
COMMENT ON COLUMN customers.updated_at IS 'Timestamp when the customer was last updated';
