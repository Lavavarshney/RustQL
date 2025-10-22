mod parser;
mod executor;

use crate::parser::{parse, tokenize};
use crate::executor::Database;

const DB_FILE: &str = "database.bin";

fn main() {
    // Try to load existing database, or create new one
    let mut db = match Database::load(DB_FILE) {
        Ok(loaded_db) => {
            println!("Loaded existing database from {}", DB_FILE);
            loaded_db
        }
        Err(_) => {
            println!("Starting with new database");
            Database::new()
        }
    };

    println!("Mini SQL Engine - Enter SQL commands (type 'quit' to exit)");
    println!("Supported commands:");
    println!("  CREATE TABLE table_name (col1, col2, ...);");
    println!("  INSERT INTO table_name VALUES (val1, val2, ...);");
    println!("  SELECT * FROM table_name;");
    println!("  SELECT col1, col2 FROM table_name;");
    println!("  UPDATE table_name SET col = value WHERE condition;");
    println!("  DELETE FROM table_name WHERE condition;");
    println!("\nSpecial commands:");
    println!("  save  - Manually save database");
    println!("  debug - Show database structure and contents");
    println!("  quit  - Save and exit");
    println!();

    loop {
        print!("> ");
        use std::io::Write;
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("quit") {
            // Auto-save on quit
            if let Err(e) = db.save(DB_FILE) {
                println!("Error saving database: {}", e);
            } else {
                println!("Database saved to {}", DB_FILE);
            }
            break;
        }

        if input.eq_ignore_ascii_case("save") {
            match db.save(DB_FILE) {
                Ok(_) => println!("Database saved to {}", DB_FILE),
                Err(e) => println!("Error saving: {}", e),
            }
            continue;
        }

        if input.eq_ignore_ascii_case("debug") {
            println!("=== Database Debug Info ===");
            println!("Tables: {}", db.tables.len());
            for (table_name, table) in &db.tables {
                println!("\nTable: {}", table_name);
                println!("  Columns: {:?}", table.columns);
                println!("  Rows: {}", table.rows.len());
                for (i, row) in table.rows.iter().enumerate() {
                    println!("    Row {}: {:?}", i, row);
                }
            }
            println!("===========================");
            continue;
        }

        if input.is_empty() {
            continue;
        }

        let tokens = tokenize(input);
        match parse(&tokens) {
            Ok(statement) => {
                db.execute(statement);
                println!("OK");
                
                // Auto-save after each successful operation
                if let Err(e) = db.save(DB_FILE) {
                    println!("Warning: Could not auto-save: {}", e);
                }
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}
