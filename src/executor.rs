use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use crate::parser::{self, InsertStatement, Statement, Value};

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    pub rows: Vec<Vec<Value>>,
    pub columns: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub tables: HashMap<String, Table>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            tables: HashMap::new(),
        }
    }

    pub fn execute(&mut self, stmt: Statement) {
        match stmt {
            Statement::Insert(insert_stmt) => self.execute_insert(insert_stmt),
            Statement::Select(select_stmt) => self.execute_select(select_stmt),
            Statement::Create(create_stmt) => self.execute_create(create_stmt),
            Statement::Delete(delete_stmt) => self.execute_delete(delete_stmt),
            Statement::Update(update_stmt) => self.execute_update(update_stmt),
        }
    }

    fn execute_insert(&mut self, insert_stmt: InsertStatement) {
        let table = self
            .tables
            .entry(insert_stmt.table_name.clone())
            .or_insert(Table { rows: vec![], columns: vec![] });

        table.rows.push(insert_stmt.values);
    }
    fn execute_select(&self, select_stmt: parser::SelectStatement) {
    let table = match self.tables.get(&select_stmt.table_name) {
        Some(t) => t,
        None => {
            println!("Table '{}' not found", select_stmt.table_name);
            return;
        }
    };

    if table.rows.is_empty() {
        println!("No rows found in table '{}'", select_stmt.table_name);
        return;
    }

    // Check if SELECT *
    if select_stmt.values.len() == 1 {
        if let Value::Star = select_stmt.values[0] {
            // Print all columns
            for row in &table.rows {
                let row_str: Vec<String> = row.iter()
                    .map(|v| match v {
                        Value::Int(i) => i.to_string(),
                        Value::Str(s) => format!("'{}'", s),
                        _ => String::from("NULL"),
                    })
                    .collect();
                println!("{:?}", row_str);
            }
            return;
        }
    }

    // Otherwise SELECT specific columns
    for row in &table.rows {
        let mut selected = Vec::new();
        for val in &select_stmt.values {
            match val {
                Value::Identifier(name) => {
                    // Try to find column by name
                    let col_index = if let Some(idx) = table.columns.iter().position(|c| c == name) {
                        idx
                    } else if name.starts_with("col") {
                        name[3..].parse::<usize>().unwrap_or(0)
                    } else {
                        0
                    };
                    
                    if col_index < row.len() {
                        match &row[col_index] {
                            Value::Int(i) => selected.push(i.to_string()),
                            Value::Str(s) => selected.push(format!("'{}'", s)),
                            _ => selected.push(String::from("NULL")),
                        }
                    }
                }
                _ => {}
            }
        }
        if !selected.is_empty() {
            println!("{:?}", selected);
        }
    }
}
fn execute_create(&mut self, create_stmt: parser::CreateTableStatement) {
        self.tables.insert(
            create_stmt.table_name, 
            Table { 
                rows: vec![], 
                columns: create_stmt.columns 
            }
        );
    }

    fn execute_delete(&mut self, delete_stmt: parser::DeleteStatement) {
        if let Some(table) = self.tables.get_mut(&delete_stmt.table_name) {
            let cond = delete_stmt.condition.trim();
            println!("Delete condition: '{}'", cond);
            if cond.is_empty() {
                println!("No condition provided. Nothing to delete.");
                return;
            }
            if let Some(eq_pos) = cond.find('=') {
                let (col_part, val_part) = cond.split_at(eq_pos);
                let col_part = col_part.trim();
                let val_part = val_part[1..].trim(); // skip '='

                // Resolve column name or index
                let col_index = if col_part.starts_with("col") {
                    col_part[3..].parse::<usize>().unwrap_or(0)
                } else {
                    // Try to find column by name
                    table.columns.iter().position(|c| c == col_part).unwrap_or(0)
                };
                println!("Column index: {}", col_index);

                let cond_value = if val_part.starts_with("'") && val_part.ends_with("'") {
                    Value::Str(val_part.trim_matches('\'').to_string())
                } else if let Ok(i) = val_part.parse::<i32>() {
                    Value::Int(i)
                } else {
                    Value::Str(val_part.to_string())
                };
                println!("Condition value: {:?}", cond_value);

                let before = table.rows.len();
                table.rows.retain(|row| {
                    if let Some(row_val) = row.get(col_index) {
                        println!("Checking row value: {:?} against {:?}", row_val, cond_value);
                        row_val != &cond_value
                    } else {
                        true
                    }
                });
                let after = table.rows.len();
                println!("Rows before: {}, after: {}", before, after);
            } else {
                println!("No '=' found in condition. Nothing deleted.");
            }
        } else {
            println!("Table '{}' not found.", delete_stmt.table_name);
        }
    }

    fn execute_update(&mut self, update_stmt: parser::UpdateStatement) {
        if let Some(table) = self.tables.get_mut(&update_stmt.table_name) {
            // Parse SET clause (e.g., "col0 = 123" or "name = 'Bob'")
            let set_parts: Vec<&str> = update_stmt.set_clause.split('=').collect();
            if set_parts.len() != 2 {
                println!("Invalid SET clause format");
                return;
            }
            
            let set_col = set_parts[0].trim();
            let set_val = set_parts[1].trim();
            
            // Resolve SET column
            let set_col_index = if set_col.starts_with("col") {
                set_col[3..].parse::<usize>().unwrap_or(0)
            } else {
                table.columns.iter().position(|c| c == set_col).unwrap_or(0)
            };
            
            // Parse SET value
            let new_value = if set_val.starts_with("'") && set_val.ends_with("'") {
                Value::Str(set_val.trim_matches('\'').to_string())
            } else if let Ok(i) = set_val.parse::<i32>() {
                Value::Int(i)
            } else {
                Value::Str(set_val.to_string())
            };

            // Parse WHERE condition if present
            let cond = update_stmt.condition.trim();
            if cond.is_empty() {
                // Update all rows
                for row in &mut table.rows {
                    if let Some(cell) = row.get_mut(set_col_index) {
                        *cell = new_value.clone();
                    }
                }
                println!("Updated all rows");
            } else if let Some(eq_pos) = cond.find('=') {
                let (col_part, val_part) = cond.split_at(eq_pos);
                let col_part = col_part.trim();
                let val_part = val_part[1..].trim();

                // Resolve WHERE column
                let where_col_index = if col_part.starts_with("col") {
                    col_part[3..].parse::<usize>().unwrap_or(0)
                } else {
                    table.columns.iter().position(|c| c == col_part).unwrap_or(0)
                };

                // Parse WHERE value
                let cond_value = if val_part.starts_with("'") && val_part.ends_with("'") {
                    Value::Str(val_part.trim_matches('\'').to_string())
                } else if let Ok(i) = val_part.parse::<i32>() {
                    Value::Int(i)
                } else {
                    Value::Str(val_part.to_string())
                };

                // Update matching rows
                let mut count = 0;
                for row in &mut table.rows {
                    if let Some(row_val) = row.get(where_col_index) {
                        if row_val == &cond_value {
                            if let Some(cell) = row.get_mut(set_col_index) {
                                *cell = new_value.clone();
                                count += 1;
                            }
                        }
                    }
                }
                println!("Updated {} rows", count);
            }
        } else {
            println!("Table '{}' not found", update_stmt.table_name);
        }
    }

    // Save database to file
    pub fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let encoded = bincode::serialize(&self)?;
        std::fs::write(path, encoded)?;
        Ok(())
    }

    // Load database from file
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let data = std::fs::read(path)?;
        let db = bincode::deserialize(&data)?;
        Ok(db)
    }
}
