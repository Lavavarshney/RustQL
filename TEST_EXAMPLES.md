# Mini SQL Engine - Test Examples

## All Implemented Features

### 1. CREATE TABLE (with column names)
```sql
CREATE TABLE users (id, name, email);
```

### 2. INSERT INTO
```sql
INSERT INTO users VALUES (101, 'Alice', 'alice@example.com');
INSERT INTO users VALUES (102, 'Bob', 'bob@example.com');
INSERT INTO users VALUES (103, 'Charlie', 'charlie@example.com');
```

### 3. SELECT with * (all columns)
```sql
SELECT * FROM users;
```

### 4. SELECT with column names (not just col0, col1)
```sql
SELECT id FROM users;
SELECT name, email FROM users;
SELECT id, name FROM users;
```

### 5. UPDATE (NEW FEATURE!)
Update a specific row by column name:
```sql
UPDATE users SET name = 'Robert' WHERE id = 102;
```

Update using col0 syntax:
```sql
UPDATE users SET col1 = 'Bobby' WHERE col0 = 102;
```

Update all rows (no WHERE clause):
```sql
UPDATE users SET email = 'updated@example.com' WHERE id = 999;
```

### 6. DELETE with column names (not just col0)
Delete by column name:
```sql
DELETE FROM users WHERE name = 'Charlie';
```

Delete by id:
```sql
DELETE FROM users WHERE id = 103;
```

Delete using col0 syntax (still supported):
```sql
DELETE FROM users WHERE col0 = 101;
```

### 7. PERSISTENCE (NEW FEATURE!)
Data is automatically saved to `database.bin` after every operation.

When you quit and restart, your data is automatically loaded:
```
quit
```

Then run again:
```
cargo run
```

You'll see: "Loaded existing database from database.bin"

Manual save command:
```
save
```

## Complete Test Workflow

Here's a complete test sequence:

```sql
CREATE TABLE products (product_id, product_name, price);
INSERT INTO products VALUES (1, 'Laptop', 999);
INSERT INTO products VALUES (2, 'Mouse', 25);
INSERT INTO products VALUES (3, 'Keyboard', 75);
SELECT * FROM products;
SELECT product_name, price FROM products;
UPDATE products SET price = 899 WHERE product_id = 1;
SELECT * FROM products;
DELETE FROM products WHERE product_name = 'Mouse';
SELECT * FROM products;
save
quit
```

## Key Improvements

1. ✅ **UPDATE support**: Can now modify existing rows
2. ✅ **Column name references**: Use actual column names (id, name, email) instead of just col0, col1
3. ✅ **Persistence**: Data automatically saves and loads between sessions
4. ✅ **Better WHERE clauses**: Works with both column names and col0/col1 syntax

## Notes

- The engine still uses a simple equality-only WHERE clause (column = value)
- Complex WHERE clauses with AND/OR are not yet supported
- Column names are case-sensitive
- Persistence uses binary serialization for efficiency
