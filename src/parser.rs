use crate::{
    ast::{Statement, AST},
    lexer::{Lexer, Token, TokenType},
};

struct Parser {
    lexer: Lexer,
    curr_token: Token,
    next_token: Token,
}

impl Parser {
    fn new(mut lexer: Lexer) -> Self {
        let curr_token = lexer.next_token();
        let next_token = lexer.next_token();

        Self {
            lexer,
            curr_token,
            next_token,
        }
    }

    fn next_token(&mut self) {
        self.curr_token = self.next_token.clone();
        self.next_token = self.lexer.next_token();
    }

    fn parse(&mut self) -> AST {
        let mut ast = AST::new();

        while self.curr_token.token_type != TokenType::EOF {
            if let Some(statement) = self.parse_statement() {
                ast.statements.push(statement);
            }
            self.next_token();
        }

        ast
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.curr_token.token_type {
            TokenType::LParen => None,
            TokenType::RParen => None,
            TokenType::LSqBrace => None,
            TokenType::RSqBrace => None,
            TokenType::Equals => None,
            TokenType::Comma => None,
            TokenType::Layout => self.parser_layout_statement(),
            TokenType::Blank => None,
            TokenType::Unknown => None,
            TokenType::EOF => None,
        }
    }

    fn parser_layout_statement(&self) -> Option<Statement> {
        if self.next_token.token_type != TokenType::LParen {
            return None;
        }

        Some(Statement::new("Layout"))
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse() {
        let content = r##"[_QWERTY] = LAYOUT(
  KC_ESC  , KC_Q , _____ , KC_E 
  ),"##
            .to_string();

        let lexer = Lexer::new(content);
        let mut parser = Parser::new(lexer);

        let ast = parser.parse();

        assert_eq!(
            "Layout",
            ast.statements
                .get(0)
                .expect("Failed to find statement in ast")
                .name
        );
    }
}
