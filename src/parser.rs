use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub enum Token {
    Insert,
    Into,
    Values,
    LeftParen,
    RightParen,
    Comma,
    Equals,
    Semicolon,
    String(String),
    Int(i32),
    Identifier(String),
    Select,
    From,
    Star, // Represents '*'
    Delete,
    Create,
    Update,
    Set,
    Where,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Int(i32),
    Str(String),
    Star,
    Identifier(String),
}

#[derive(Debug, Clone)]
pub struct InsertStatement {
    pub table_name: String,
    pub values: Vec<Value>,
}
#[derive(Debug, Clone)]
pub struct SelectStatement {
    pub table_name: String,
    pub values: Vec<Value>,
}
#[derive(Debug, Clone)]
pub struct CreateTableStatement {
    pub table_name: String,
    pub columns: Vec<String>,
}
#[derive(Debug, Clone)]
pub struct DeleteStatement {
    pub table_name: String,
    pub condition: String,
}
#[derive(Debug, Clone)]
pub struct UpdateStatement {
    pub table_name: String,
    pub set_clause: String,  // e.g., "col0 = 123"
    pub condition: String,    // e.g., "col1 = 'Alice'"
}
#[derive(Debug, Clone)]
pub enum Statement {
    Insert(InsertStatement),
    Select(SelectStatement),
    Create(CreateTableStatement),
    Delete(DeleteStatement),
    Update(UpdateStatement),
}

// --- Tokenizer ---
pub fn tokenize(input: &str) -> Vec<Token> {
    let mut chars = input.chars().peekable();
    let mut tokens = vec![];

    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' | '\n' => {
                chars.next(); // skip whitespace
            }
            '(' => {
                tokens.push(Token::LeftParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RightParen);
                chars.next();
            }
            ',' => {
                tokens.push(Token::Comma);
                chars.next();
            }
            ';' => {
                tokens.push(Token::Semicolon);
                chars.next();
            }
            '=' => {
        tokens.push(Token::Equals);
        chars.next();
    }
             '*' => {
            tokens.push(Token::Star);
            chars.next();
        }
            '\'' => {
                chars.next(); // skip opening '
                let mut s = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch == '\'' {
                        chars.next(); // skip closing '
                        break;
                    }
                    s.push(ch);
                    chars.next();
                }
                tokens.push(Token::String(s));
            }
            c if c.is_ascii_digit() => {
                let mut num = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_ascii_digit() {
                        num.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let parsed = num.parse::<i32>().expect("Invalid integer");
                tokens.push(Token::Int(parsed));
            }
            c if c.is_ascii_alphabetic() || c == '_' => {
                let mut word = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        word.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                match word.to_uppercase().as_str() {
                    "INSERT" => tokens.push(Token::Insert),
                    "INTO" => tokens.push(Token::Into),
                    "VALUES" => tokens.push(Token::Values),
                    "SELECT" => tokens.push(Token::Select),
                    "*" => tokens.push(Token::Star),
                    "FROM" => tokens.push(Token::From),
                    "DELETE" => tokens.push(Token::Delete),
                    "CREATE" => tokens.push(Token::Create),
                    "UPDATE" => tokens.push(Token::Update),
                    "SET" => tokens.push(Token::Set),
                    "WHERE" => tokens.push(Token::Where),
                    _ => tokens.push(Token::Identifier(word)),
                }
            }
            _ => {
                panic!("Unexpected character: {}", c);
            }
        }
    }

    tokens
}

// --- Parser ---
pub fn parse(tokens: &[Token]) -> Result<Statement, String> {
    return match tokens.first() {
        Some(Token::Insert) => parse_insert(tokens),
        Some(Token::Select) => parse_select(tokens),
        Some(Token::Create) => parse_create(tokens),
        Some(Token::Delete) => parse_delete(tokens),
        Some(Token::Update) => parse_update(tokens),
        _ => Err("Unknown or unsupported statement".into()),
    };
}
// Parses: CREATE TABLE table_name (col1, col2, ...);
pub fn parse_create(tokens: &[Token]) -> Result<Statement, String> {
    let mut iter = tokens.iter().peekable();

    match iter.next() {
        Some(Token::Create) => {}
        _ => return Err("Expected 'CREATE'".into()),
    }

    match iter.next() {
        Some(Token::Identifier(kw)) if kw.to_uppercase() == "TABLE" => {},
        _ => return Err("Expected 'TABLE' after 'CREATE'".into()),
    }

    let table_name = match iter.next() {
        Some(Token::Identifier(name)) => name.clone(),
        _ => return Err("Expected table name after 'TABLE'".into()),
    };

    match iter.next() {
        Some(Token::LeftParen) => {}
        _ => return Err("Expected '(' after table name".into()),
    }

    let mut columns = vec![];
    loop {
        match iter.next() {
            Some(Token::Identifier(col)) => columns.push(col.clone()),
            Some(Token::Comma) => continue,
            Some(Token::RightParen) => break,
            Some(tok) => return Err(format!("Unexpected token in columns: {:?}", tok)),
            None => return Err("Unexpected end of input in columns".into()),
        }
    }

    if let Some(Token::Semicolon) = iter.peek() {
        iter.next(); // consume semicolon
    }

    Ok(Statement::Create(CreateTableStatement { table_name, columns }))
}

// Parses: DELETE FROM table_name WHERE condition;
pub fn parse_delete(tokens: &[Token]) -> Result<Statement, String> {
    let mut iter = tokens.iter().peekable();

    match iter.next() {
        Some(Token::Delete) => {}
        _ => return Err("Expected 'DELETE'".into()),
    }

    match iter.next() {
    Some(Token::From) => {},
    _ => return Err("Expected 'FROM' after 'DELETE'".into()),
}

    let table_name = match iter.next() {
        Some(Token::Identifier(name)) => name.clone(),
        _ => return Err("Expected table name after 'FROM'".into()),
    };

    let condition = match iter.next() {
        Some(Token::Where) => {
            // Collect everything until semicolon as condition string
            let mut cond = String::new();
            while let Some(tok) = iter.next() {
                match tok {
                    Token::Semicolon => break,
                    Token::Identifier(s) => {
                        if !cond.is_empty() { cond.push(' '); }
                        cond.push_str(s);
                    }
                    Token::Equals => {
                        cond.push_str(" = ");
                    }
                    Token::String(s) => {
                        if !cond.is_empty() { cond.push(' '); }
                        cond.push_str(&format!("'{}'", s));
                    }
                    Token::Int(i) => {
                        if !cond.is_empty() { cond.push(' '); }
                        cond.push_str(&i.to_string());
                    }
                    Token::Star => {
                        if !cond.is_empty() { cond.push(' '); }
                        cond.push('*');
                    }
                    Token::Comma => cond.push(','),
                    _ => {}
                }
            }
            cond
        }
        Some(Token::Identifier(kw)) if kw.to_uppercase() == "WHERE" => {
            // Backwards compatibility
            let mut cond = String::new();
            while let Some(tok) = iter.next() {
                match tok {
                    Token::Semicolon => break,
                    Token::Identifier(s) => {
                        if !cond.is_empty() { cond.push(' '); }
                        cond.push_str(s);
                    }
                    Token::Equals => {
                        cond.push_str(" = ");
                    }
                    Token::String(s) => {
                        if !cond.is_empty() { cond.push(' '); }
                        cond.push_str(&format!("'{}'", s));
                    }
                    Token::Int(i) => {
                        if !cond.is_empty() { cond.push(' '); }
                        cond.push_str(&i.to_string());
                    }
                    Token::Star => {
                        if !cond.is_empty() { cond.push(' '); }
                        cond.push('*');
                    }
                    Token::Comma => cond.push(','),
                    _ => {}
                }
            }
            cond
        }
        _ => return Err("Expected 'WHERE' after table name in DELETE".into()),
    };

    Ok(Statement::Delete(DeleteStatement { table_name, condition }))
}


pub fn parse_insert(tokens: &[Token]) -> Result<Statement, String> {
    let mut iter = tokens.iter().peekable();

    match iter.next() {
        Some(Token::Insert) => {}
        _ => return Err("Expected 'INSERT'".into()),
    }

    match iter.next() {
        Some(Token::Into) => {}
        _ => return Err("Expected 'INTO' after 'INSERT'".into()),
    }

    let table_name = match iter.next() {
        Some(Token::Identifier(name)) => name.clone(),
        _ => return Err("Expected table name after 'INTO'".into()),
    };

    match iter.next() {
        Some(Token::Values) => {}
        _ => return Err("Expected 'VALUES' keyword".into()),
    }

    match iter.next() {
        Some(Token::LeftParen) => {}
        _ => return Err("Expected '(' after 'VALUES'".into()),
    }

    let mut values = vec![];

    loop {
        match iter.next() {
            Some(Token::Int(i)) => values.push(Value::Int(*i)),
            Some(Token::String(s)) => values.push(Value::Str(s.clone())),
            Some(Token::Comma) => continue,
            Some(Token::RightParen) => break,
            Some(tok) => return Err(format!("Unexpected token in VALUES: {:?}", tok)),
            None => return Err("Unexpected end of input in VALUES".into()),
        }
    }

    if let Some(Token::Semicolon) = iter.peek() {
        iter.next(); // consume semicolon
    }
     Ok(Statement::Insert(InsertStatement { table_name, values }))
    
}
pub fn parse_select(tokens: &[Token]) -> Result<Statement, String> {
    let mut iter = tokens.iter().peekable();

    match iter.next() {
        Some(Token::Select) => {}
        _ => return Err("Expected 'SELECT'".into()),
    }

    let mut values = vec![];

    loop {
        match iter.next() {
            Some(Token::Star) => values.push(Value::Star),
            Some(Token::Identifier(name)) => values.push(Value::Identifier(name.clone())),
            Some(Token::Comma) => continue,
            Some(Token::From) => break,
            Some(tok) => return Err(format!("Unexpected token in SELECT: {:?}", tok)),
            None => return Err("Unexpected end of input in SELECT".into()),
        }
    }

    let table_name = match iter.next() {
        Some(Token::Identifier(name)) => name.clone(),
        _ => return Err("Expected table name after 'FROM'".into()),
    };

    if let Some(Token::Semicolon) = iter.peek() {
        iter.next(); // consume semicolon
    }

    Ok(Statement::Select(SelectStatement { table_name, values }))
}

// Parses: UPDATE table_name SET col0 = value WHERE condition;
pub fn parse_update(tokens: &[Token]) -> Result<Statement, String> {
    let mut iter = tokens.iter().peekable();

    match iter.next() {
        Some(Token::Update) => {}
        _ => return Err("Expected 'UPDATE'".into()),
    }

    let table_name = match iter.next() {
        Some(Token::Identifier(name)) => name.clone(),
        _ => return Err("Expected table name after 'UPDATE'".into()),
    };

    match iter.next() {
        Some(Token::Set) => {}
        _ => return Err("Expected 'SET' after table name".into()),
    }

    // Collect SET clause until WHERE
    let mut set_clause = String::new();
    loop {
        match iter.next() {
            Some(Token::Where) => break,
            Some(Token::Identifier(kw)) if kw.to_uppercase() == "WHERE" => break,
            Some(Token::Identifier(s)) => {
                if !set_clause.is_empty() { set_clause.push(' '); }
                set_clause.push_str(s);
            }
            Some(Token::Equals) => {
                set_clause.push_str(" = ");
            }
            Some(Token::String(s)) => {
                if !set_clause.is_empty() { set_clause.push(' '); }
                set_clause.push_str(&format!("'{}'", s));
            }
            Some(Token::Int(i)) => {
                if !set_clause.is_empty() { set_clause.push(' '); }
                set_clause.push_str(&i.to_string());
            }
            Some(Token::Comma) => set_clause.push(','),
            Some(Token::Semicolon) => return Ok(Statement::Update(UpdateStatement { 
                table_name, 
                set_clause, 
                condition: String::new() 
            })),
            Some(tok) => return Err(format!("Unexpected token in SET clause: {:?}", tok)),
            None => return Err("Unexpected end of input in SET clause".into()),
        }
    }

    // Collect WHERE condition until semicolon
    let mut condition = String::new();
    while let Some(tok) = iter.next() {
        match tok {
            Token::Semicolon => break,
            Token::Identifier(s) => {
                if !condition.is_empty() { condition.push(' '); }
                condition.push_str(s);
            }
            Token::Equals => {
                condition.push_str(" = ");
            }
            Token::String(s) => {
                if !condition.is_empty() { condition.push(' '); }
                condition.push_str(&format!("'{}'", s));
            }
            Token::Int(i) => {
                if !condition.is_empty() { condition.push(' '); }
                condition.push_str(&i.to_string());
            }
            Token::Comma => condition.push(','),
            _ => {}
        }
    }

    Ok(Statement::Update(UpdateStatement { table_name, set_clause, condition }))
}