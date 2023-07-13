pub struct AST {
    pub statements: Vec<Statement>,
}

impl AST {
    pub fn new() -> Self {
        Self { statements: vec![] }
    }
}

pub struct Statement {
    pub name: String,
}

impl Statement {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}
