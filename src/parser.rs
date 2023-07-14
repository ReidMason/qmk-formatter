use crate::{
    ast::{KeymapStatement, LayoutStatement, StatementEnum, AST},
    lexer::{Lexer, TokenType},
};

pub struct Parser {
    lexer: Lexer,
    curr_token: TokenType,
    next_token: TokenType,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
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

    pub fn parse(&mut self) -> AST {
        let mut ast = AST::new();

        while self.curr_token != TokenType::EOF {
            if let Some(statement) = self.parse_statement() {
                ast.statements.push(statement);
            }
            self.next_token();
        }

        ast
    }

    fn parse_statement(&mut self) -> Option<StatementEnum> {
        match self.curr_token {
            TokenType::LParen(..) => None,
            TokenType::RParen(..) => None,
            TokenType::LSqBrace(..) => self.parse_keymap_statement(),
            TokenType::RSqBrace(..) => None,
            TokenType::Equals(..) => None,
            TokenType::Comma(..) => None,
            TokenType::Layout(..) => None,
            TokenType::Blank(..) => None,
            TokenType::Unknown(..) => None,
            TokenType::EOF => None,
        }
    }

    fn parse_keymap_statement(&mut self) -> Option<StatementEnum> {
        match self.next_token {
            TokenType::Unknown(..) => {}
            _ => return None,
        }

        self.next_token(); // Curr: _QWERTY

        match self.next_token {
            TokenType::RSqBrace(..) => {}
            _ => return None,
        }

        let token = self.curr_token.clone();

        self.next_token(); // Curr: ]

        let layout_statement = match self.parse_layout_statement() {
            Some(x) => x,
            None => return None,
        };

        let statement = KeymapStatement::new(token, layout_statement);
        let statement = StatementEnum::KeymapStatement(statement);
        Some(statement)
    }

    fn parse_layout_statement(&mut self) -> Option<LayoutStatement> {
        match self.next_token {
            TokenType::Equals(..) => {}
            _ => return None,
        }

        self.next_token(); // Curr: =

        match self.next_token {
            TokenType::Layout(..) => {}
            _ => return None,
        }

        self.next_token(); // Curr: Layout

        match self.next_token {
            TokenType::LParen(..) => {}
            _ => return None,
        }

        let token = self.curr_token.clone();

        self.next_token(); // Curr: (

        let keys = self.parse_layout_keys();
        let statement = LayoutStatement::new(token, keys);

        return Some(statement);
    }

    fn parse_layout_keys(&mut self) -> Vec<String> {
        self.next_token(); // Curr: KC_ESC

        let mut keys: Vec<String> = vec![];
        // while self.curr_token != TokenType::RParen {
        while match self.curr_token {
            TokenType::RParen(..) => false,
            _ => true,
        } {
            match &self.curr_token {
                TokenType::Unknown(_, x) => keys.push(x.to_string()),
                TokenType::Blank(..) => keys.push("".to_string()),
                _ => {}
            };

            self.next_token();
        }

        keys
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::LayoutStatement;

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
            &StatementEnum::KeymapStatement(KeymapStatement::new(
                TokenType::Unknown(1, "_QWERTY".to_string()),
                LayoutStatement::new(
                    TokenType::Layout(12),
                    vec![
                        "KC_ESC".to_string(),
                        "KC_Q".to_string(),
                        "".to_string(),
                        "KC_E".to_string()
                    ]
                )
            )),
            ast.statements
                .get(0)
                .expect("Failed to find statement in ast")
        );
    }
}
