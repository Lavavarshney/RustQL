# Complete Demo Session

This file shows a complete demonstration of all features.

## Session 1: Creating and Populating Data

```
$ cargo run
Starting with new database
Mini SQL Engine - Enter SQL commands (type 'quit' to exit, 'save' to save manually)
...

> CREATE TABLE employees (id, name, department, salary);
OK

> INSERT INTO employees VALUES (1, 'Alice', 'Engineering', 95000);
OK

> INSERT INTO employees VALUES (2, 'Bob', 'Sales', 75000);
OK

> INSERT INTO employees VALUES (3, 'Charlie', 'Engineering', 85000);
OK

> INSERT INTO employees VALUES (4, 'Diana', 'HR', 70000);
OK

> SELECT * FROM employees;
["1", "Alice", "Engineering", "95000"]
["2", "Bob", "Sales", "75000"]
["3", "Charlie", "Engineering", "85000"]
["4", "Diana", "HR", "70000"]
OK

> SELECT name, department FROM employees;
["Alice", "Engineering"]
["Bob", "Sales"]
["Charlie", "Engineering"]
["Diana", "HR"]
OK

> quit
Database saved to database.bin
```

## Session 2: Loading Persisted Data and Using UPDATE

```
$ cargo run
Loaded existing database from database.bin
...

> SELECT * FROM employees;
["1", "Alice", "Engineering", "95000"]
["2", "Bob", "Sales", "75000"]
["3", "Charlie", "Engineering", "85000"]
["4", "Diana", "HR", "70000"]
OK

> UPDATE employees SET salary = 100000 WHERE name = 'Alice';
Updated 1 rows
OK

> UPDATE employees SET department = 'Marketing' WHERE id = 2;
Updated 1 rows
OK

> SELECT * FROM employees;
["1", "Alice", "Engineering", "100000"]
["2", "Bob", "Marketing", "75000"]
["3", "Charlie", "Engineering", "85000"]
["4", "Diana", "HR", "70000"]
OK

> SELECT name, salary FROM employees;
["Alice", "100000"]
["Bob", "75000"]
["Charlie", "85000"]
["Diana", "70000"]
OK

> quit
Database saved to database.bin
```

## Session 3: Using DELETE with Column Names

```
$ cargo run
Loaded existing database from database.bin
...

> SELECT * FROM employees;
["1", "Alice", "Engineering", "100000"]
["2", "Bob", "Marketing", "75000"]
["3", "Charlie", "Engineering", "85000"]
["4", "Diana", "HR", "70000"]
OK

> DELETE FROM employees WHERE department = 'Marketing';
Delete condition: 'department = Marketing'
Column index: 2
Condition value: Str("Marketing")
Checking row value: Str("Engineering") against Str("Marketing")
Checking row value: Str("Marketing") against Str("Marketing")
Checking row value: Str("Engineering") against Str("Marketing")
Checking row value: Str("HR") against Str("Marketing")
Rows before: 4, after: 3
OK

> SELECT * FROM employees;
["1", "Alice", "Engineering", "100000"]
["3", "Charlie", "Engineering", "85000"]
["4", "Diana", "HR", "70000"]
OK

> DELETE FROM employees WHERE id = 4;
Delete condition: 'id = 4'
Column index: 0
Condition value: Int(4)
Checking row value: Int(1) against Int(4)
Checking row value: Int(3) against Int(4)
Checking row value: Int(4) against Int(4)
Rows before: 3, after: 2
OK

> SELECT name, department, salary FROM employees;
["Alice", "Engineering", "100000"]
["Charlie", "Engineering", "85000"]
OK

> quit
Database saved to database.bin
```

## Session 4: Complex Operations

```
$ cargo run
Loaded existing database from database.bin
...

> CREATE TABLE products (product_id, product_name, category, price, stock);
OK

> INSERT INTO products VALUES (101, 'Laptop', 'Electronics', 999, 50);
OK

> INSERT INTO products VALUES (102, 'Mouse', 'Electronics', 25, 200);
OK

> INSERT INTO products VALUES (103, 'Desk', 'Furniture', 299, 30);
OK

> INSERT INTO products VALUES (104, 'Chair', 'Furniture', 199, 45);
OK

> SELECT product_name, price FROM products;
["Laptop", "999"]
["Mouse", "25"]
["Desk", "299"]
["Chair", "199"]
OK

> UPDATE products SET price = 899 WHERE product_name = 'Laptop';
Updated 1 rows
OK

> UPDATE products SET stock = 250 WHERE product_id = 102;
Updated 1 rows
OK

> SELECT product_name, price, stock FROM products;
["Laptop", "899", "50"]
["Mouse", "25", "250"]
["Desk", "299", "30"]
["Chair", "199", "45"]
OK

> DELETE FROM products WHERE category = 'Furniture';
Delete condition: 'category = Furniture'
Column index: 2
Condition value: Str("Furniture")
...
Rows before: 4, after: 2
OK

> SELECT * FROM products;
["101", "Laptop", "Electronics", "899", "50"]
["102", "Mouse", "Electronics", "25", "250"]
OK

> save
Database saved to database.bin

> quit
Database saved to database.bin
```

## Key Observations

### 1. Persistence Works Perfectly ✅
- Data survives between sessions
- Automatic save/load on start/quit
- Manual save command available

### 2. Column Names Work ✅
- Can use `name`, `department`, `salary` instead of `col0`, `col1`, `col2`
- Works in SELECT, UPDATE, DELETE, WHERE clauses
- More intuitive and readable

### 3. UPDATE Works ✅
- Can modify specific rows with WHERE clause
- Supports both column names and positional syntax
- Shows update count

### 4. DELETE Enhanced ✅
- Can delete by column name (not just col0)
- Shows detailed debug info
- Reports rows affected

### 5. All CRUD Operations Supported ✅
- **C**reate: `CREATE TABLE`, `INSERT INTO`
- **R**ead: `SELECT *`, `SELECT columns`
- **U**pdate: `UPDATE ... SET ... WHERE ...`
- **D**elete: `DELETE FROM ... WHERE ...`

## Summary

Your mini SQL engine is now a fully functional, persistent database system with complete CRUD support and intuitive column name handling!
