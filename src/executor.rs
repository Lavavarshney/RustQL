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

        // Build headers and rows as strings
        let mut headers: Vec<String> = Vec::new();
        let mut rows_out: Vec<Vec<String>> = Vec::new();

        // If SELECT * -> headers are table.columns (fallback to colN if empty)
        if select_stmt.values.len() == 1 {
            if let Value::Star = select_stmt.values[0] {
                if !table.columns.is_empty() {
                    headers = table.columns.clone();
                } else {
                    headers = (0..table.rows[0].len()).map(|i| format!("col{}", i)).collect();
                }

                for row in &table.rows {
                    let row_str: Vec<String> = row.iter().map(|v| match v {
                        Value::Int(i) => i.to_string(),
                        Value::Str(s) => s.clone(),
                        _ => String::from("NULL"),
                    }).collect();
                    rows_out.push(row_str);
                }

                self.print_table(&headers, &rows_out);
                return;
            }
        }

        // Otherwise explicit column selection
        // Build headers from requested identifiers
        for val in &select_stmt.values {
            match val {
                Value::Identifier(name) => {
                    // Resolve to actual column name if possible
                    if name.starts_with("col") {
                        // positional
                        if let Ok(idx) = name[3..].parse::<usize>() {
                            if idx < table.columns.len() {
                                headers.push(table.columns[idx].clone());
                            } else {
                                headers.push(name.clone());
                            }
                        } else {
                            headers.push(name.clone());
                        }
                    } else if let Some(idx) = table.columns.iter().position(|c| c == name) {
                        headers.push(table.columns[idx].clone());
                    } else {
                        headers.push(name.clone());
                    }
                }
                _ => {}
            }
        }

        // For each row, extract the requested columns
        for row in &table.rows {
            let mut row_strs: Vec<String> = Vec::new();
            for val in &select_stmt.values {
                match val {
                    Value::Identifier(name) => {
                        let col_index = if name.starts_with("col") {
                            name[3..].parse::<usize>().unwrap_or(0)
                        } else {
                            table.columns.iter().position(|c| c == name).unwrap_or(0)
                        };
                        if let Some(cell) = row.get(col_index) {
                            match cell {
                                Value::Int(i) => row_strs.push(i.to_string()),
                                Value::Str(s) => row_strs.push(s.clone()),
                                _ => row_strs.push(String::from("NULL")),
                            }
                        } else {
                            row_strs.push(String::new());
                        }
                    }
                    _ => {}
                }
            }
            rows_out.push(row_strs);
        }

        self.print_table(&headers, &rows_out);
    }

    // Helper: pretty-print table
    fn print_table(&self, headers: &[String], rows: &[Vec<String>]) {
        // compute column widths
        let cols = headers.len();
        let mut widths = headers.iter().map(|h| h.len()).collect::<Vec<usize>>();
        for row in rows {
            for (i, cell) in row.iter().enumerate().take(cols) {
                if cell.len() > widths[i] {
                    widths[i] = cell.len();
                }
            }
        }

        // horizontal border builders
        let mut sep = String::new();
        sep.push('+');
        for w in &widths {
            sep.push_str(&"-".repeat(*w + 2));
            sep.push('+');
        }

        // print header
        println!("{}", sep);
        // header row
        let mut header_row = String::from("|");
        for (i, h) in headers.iter().enumerate() {
            let pad = widths[i].saturating_sub(h.len());
            header_row.push(' ');
            header_row.push_str(h);
            header_row.push_str(&" ".repeat(pad + 1));
            header_row.push('|');
        }
        println!("{}", header_row);
        println!("{}", sep);

        // print rows
        for row in rows {
            let mut row_line = String::from("|");
            for (i, cell) in row.iter().enumerate().take(cols) {
                let pad = widths[i].saturating_sub(cell.len());
                row_line.push(' ');
                row_line.push_str(cell);
                row_line.push_str(&" ".repeat(pad + 1));
                row_line.push('|');
            }
            println!("{}", row_line);
        }
        println!("{}", sep);
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
