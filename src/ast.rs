use crate::lexer::Token;

pub struct AST {
    pub statements: Vec<StatementEnum>,
}

#[derive(Debug, PartialEq)]
pub enum StatementEnum {
    KeymapStatement(KeymapStatement),
}

#[derive(Debug, PartialEq)]
pub struct KeymapStatement {
    pub token: Token,
    pub layout_statement: LayoutStatement,
}

impl KeymapStatement {
    pub fn new(token: Token, layout_statement: LayoutStatement) -> Self {
        Self {
            token,
            layout_statement,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct LayoutStatement {
    token: Token,
    pub keys: Vec<String>,
}

impl LayoutStatement {
    pub fn new(token: Token, keys: Vec<String>) -> Self {
        Self { token, keys }
    }
}

impl AST {
    pub fn new() -> Self {
        Self { statements: vec![] }
    }
}
