# Mini SQL Engine - Quick Reference

## Supported SQL Commands

### CREATE TABLE
```sql
CREATE TABLE table_name (col1, col2, col3);
```
Creates a new table with named columns.

### INSERT INTO
```sql
INSERT INTO table_name VALUES (value1, value2, value3);
```
Inserts a row. Values can be integers or strings (use single quotes for strings).

### SELECT
```sql
SELECT * FROM table_name;                    -- All columns
SELECT col1, col2 FROM table_name;           -- Specific columns by name
SELECT col0, col1 FROM table_name;           -- Specific columns by index
```

### UPDATE ✨ NEW
```sql
UPDATE table_name SET column = value WHERE condition;
UPDATE table_name SET col0 = 123 WHERE col1 = 'test';
```
Updates rows. SET and WHERE support both column names and col0/col1 syntax.

### DELETE
```sql
DELETE FROM table_name WHERE column = value;
DELETE FROM table_name WHERE col0 = 123;
```
Deletes rows matching the condition. Supports column names and col0/col1 syntax.

## Special Commands

- `save` - Manually save database to disk
- `quit` - Exit (auto-saves before quitting)

## Data Types

- **Integer**: `123`, `456`, `-10`
- **String**: `'Alice'`, `'test@example.com'`, `'Hello World'`

## Column References

You can reference columns in two ways:

1. **By name**: Use the column name from CREATE TABLE
   ```sql
   CREATE TABLE users (id, name, email);
   SELECT name FROM users;
   DELETE FROM users WHERE name = 'Alice';
   ```

2. **By position**: Use col0, col1, col2, etc.
   ```sql
   SELECT col0, col1 FROM users;  -- Same as: SELECT id, name FROM users;
   DELETE FROM users WHERE col0 = 101;
   ```

## Persistence

- Database automatically saves after every command
- Data persists between sessions in `database.bin`
- On startup, previous data is automatically loaded

## Example Session

```sql
-- Create a table
CREATE TABLE products (id, name, price);

-- Insert some data
INSERT INTO products VALUES (1, 'Laptop', 999);
INSERT INTO products VALUES (2, 'Mouse', 25);
INSERT INTO products VALUES (3, 'Keyboard', 75);

-- View all data
SELECT * FROM products;

-- View specific columns
SELECT name, price FROM products;

-- Update a price
UPDATE products SET price = 899 WHERE id = 1;

-- Delete a product
DELETE FROM products WHERE name = 'Mouse';

-- Verify changes
SELECT * FROM products;

-- Save and exit
quit
```

## Tips

- Commands are case-insensitive
- End each SQL statement with a semicolon (;)
- Use single quotes (') for string values
- Column names in WHERE clauses are case-sensitive
- The database file is saved as `database.bin` in the project directory
