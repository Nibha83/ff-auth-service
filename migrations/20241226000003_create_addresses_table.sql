-- Create addresses table
CREATE TABLE IF NOT EXISTS addresses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    customer_id UUID NOT NULL,
    name VARCHAR(255) NOT NULL,
    pincode VARCHAR(10) NOT NULL,
    phone VARCHAR(20) NOT NULL,
    address TEXT NOT NULL,
    is_default BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    -- Foreign key constraint
    CONSTRAINT fk_addresses_customer_id 
        FOREIGN KEY (customer_id) 
        REFERENCES customers(id) 
        ON DELETE CASCADE
);

-- Create index on customer_id for faster lookups
CREATE INDEX IF NOT EXISTS idx_addresses_customer_id ON addresses(customer_id);

-- Create index on pincode for location-based queries
CREATE INDEX IF NOT EXISTS idx_addresses_pincode ON addresses(pincode);

-- Create unique constraint to ensure only one default address per customer
CREATE UNIQUE INDEX IF NOT EXISTS idx_addresses_customer_default 
    ON addresses(customer_id) 
    WHERE is_default = TRUE;

-- Add comments to the table and columns
COMMENT ON TABLE addresses IS 'Stores customer addresses';
COMMENT ON COLUMN addresses.id IS 'Unique identifier for the address';
COMMENT ON COLUMN addresses.customer_id IS 'Reference to the customer who owns this address';
COMMENT ON COLUMN addresses.name IS 'Name/label for the address (e.g., Home, Office)';
COMMENT ON COLUMN addresses.pincode IS 'Postal/ZIP code for the address';
COMMENT ON COLUMN addresses.phone IS 'Contact phone number for this address';
COMMENT ON COLUMN addresses.address IS 'Full address text';
COMMENT ON COLUMN addresses.is_default IS 'Whether this is the default address for the customer';
COMMENT ON COLUMN addresses.created_at IS 'Timestamp when the address was created';
COMMENT ON COLUMN addresses.updated_at IS 'Timestamp when the address was last updated';
