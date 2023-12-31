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
            TokenType::Ident(..) => None,
            TokenType::Comment(..) => None,
            TokenType::Const(..) => self.parse_assignment(),
            TokenType::Progmem(_) => None,
            TokenType::LBrace(_) => None,
            TokenType::RBrace(_) => None,
            TokenType::EOF => None,
        }
    }

    fn expect_peek(&mut self, expected: TokenType) -> bool {
        if std::mem::discriminant(&expected) == std::mem::discriminant(&self.next_token) {
            match (&self.next_token, &expected) {
                (TokenType::Ident(_, x), TokenType::Ident(_, y)) if x != y => return false,
                (_, _) => (),
            };

            self.next_token();
            return true;
        }
        false
    }

    fn parse_assignment(&mut self) -> Option<StatementEnum> {
        let expected = [
            TokenType::Ident(0, "uint16_t".to_string()),
            TokenType::Progmem(0),
            TokenType::Ident(0, "keymaps".to_string()),
            TokenType::LSqBrace(0),
            TokenType::RSqBrace(0),
            TokenType::LSqBrace(0),
            TokenType::Ident(0, "MATRIX_ROWS".to_string()),
            TokenType::RSqBrace(0),
            TokenType::LSqBrace(0),
            TokenType::Ident(0, "MATRIX_COLS".to_string()),
            TokenType::RSqBrace(0),
            TokenType::Equals(0),
        ];

        for e in expected {
            if !self.expect_peek(e) {
                return None;
            }
        }

        let start: usize;
        match self.next_token {
            TokenType::LBrace(x) => start = x,
            _ => return None,
        }

        let mut keymaps: Vec<KeymapStatement> = vec![];
        while match self.next_token {
            TokenType::RBrace(..) => false,
            _ => true,
        } {
            self.next_token();

            match self.next_token {
                TokenType::LSqBrace(..) => {
                    self.next_token();
                    match self.parse_keymap_statement() {
                        Some(x) => match x {
                            StatementEnum::KeymapStatement(x) => keymaps.push(x),
                            StatementEnum::Keymaps(..) => {}
                        },
                        None => {}
                    }
                }
                _ => {}
            }
        }

        let end: usize;
        match self.next_token {
            TokenType::RBrace(x) => end = x,
            _ => return None,
        }

        return Some(StatementEnum::Keymaps(start, end, keymaps));
    }

    fn parse_keymap_statement(&mut self) -> Option<StatementEnum> {
        match self.next_token {
            TokenType::Ident(..) => self.next_token(),
            _ => return None,
        }

        let token: TokenType;
        match self.next_token {
            TokenType::RSqBrace(..) => {
                token = self.curr_token.clone();
                self.next_token();
            }
            _ => return None,
        }

        let layout_statement = match self.parse_layout_statement() {
            Some(x) => x,
            None => return None,
        };

        let statement = KeymapStatement::new(token, layout_statement);
        let statement = StatementEnum::KeymapStatement(statement);
        Some(statement)
    }

    fn parse_layout_statement(&mut self) -> Option<LayoutStatement> {
        let expected = [TokenType::Equals(0), TokenType::Layout(0)];

        for e in expected {
            if !self.expect_peek(e) {
                return None;
            }
        }

        let token: TokenType;
        match self.next_token {
            TokenType::LParen(..) => {
                token = self.curr_token.clone();
                self.next_token(); // Curr: (
            }
            _ => return None,
        }

        let keys = self.parse_layout_keys();
        let statement = LayoutStatement::new(token, keys);

        return Some(statement);
    }

    fn parse_layout_keys(&mut self) -> Vec<String> {
        self.next_token();

        let mut keys: Vec<String> = vec![];

        while match self.curr_token {
            TokenType::RParen(..) => false,
            _ => true,
        } {
            match &self.curr_token {
                TokenType::Ident(_, _) => {
                    let key_string = match &self.next_token {
                        TokenType::LParen(..) => self.parse_function_keymap(),
                        _ => self.curr_token.to_string(),
                    };
                    keys.push(key_string);
                }
                TokenType::Blank(..) => keys.push("".to_string()),
                TokenType::LParen(..) => {
                    keys.push("(".to_string());
                }
                _ => {}
            };

            self.next_token();
        }

        keys
    }

    fn parse_function_keymap(&mut self) -> String {
        let mut key_string = "".to_string();

        loop {
            key_string.push_str(&self.curr_token.to_string());
            self.next_token();

            match self.curr_token {
                TokenType::RParen(..) => break,
                _ => {}
            }
        }

        key_string.push_str(&self.curr_token.to_string());
        key_string
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

        let lexer = Lexer::new(&content);
        let mut parser = Parser::new(lexer);

        let ast = parser.parse();

        assert_eq!(
            &StatementEnum::KeymapStatement(KeymapStatement::new(
                TokenType::Ident(1, "_QWERTY".to_string()),
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

    #[test]
    fn test_parse_with_function() {
        let content = r##"[_QWERTY] = LAYOUT(
  KC_ESC  , LCTL(KC_1), _____ , KC_E 
  ),"##
            .to_string();

        let lexer = Lexer::new(&content);
        let mut parser = Parser::new(lexer);

        let ast = parser.parse();

        assert_eq!(
            &StatementEnum::KeymapStatement(KeymapStatement::new(
                TokenType::Ident(1, "_QWERTY".to_string()),
                LayoutStatement::new(
                    TokenType::Layout(12),
                    vec![
                        "KC_ESC".to_string(),
                        "LCTL(KC_1)".to_string(),
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

    #[test]
    fn test_parse_keymap_init() {
        let content = r##"const uint16_t PROGMEM keymaps[][MATRIX_ROWS][MATRIX_COLS] = {
                [_QWERTY] = LAYOUT(
  KC_ESC  , KC_Q , _____ , KC_E 
  ),
                [_SYM] = LAYOUT(
  KC_ESC  , KC_Q , _____ , KC_E 
  ),
  }"##
        .to_string();

        let lexer = Lexer::new(&content);
        let mut parser = Parser::new(lexer);

        let ast = parser.parse();

        assert_eq!(
            &StatementEnum::Keymaps(
                61,
                210,
                vec![
                    KeymapStatement::new(
                        TokenType::Ident(80, "_QWERTY".to_string()),
                        LayoutStatement::new(
                            TokenType::Layout(91),
                            vec![
                                "KC_ESC".to_string(),
                                "KC_Q".to_string(),
                                "".to_string(),
                                "KC_E".to_string()
                            ]
                        )
                    ),
                    KeymapStatement::new(
                        TokenType::Ident(154, "_SYM".to_string()),
                        LayoutStatement::new(
                            TokenType::Layout(162),
                            vec![
                                "KC_ESC".to_string(),
                                "KC_Q".to_string(),
                                "".to_string(),
                                "KC_E".to_string()
                            ]
                        )
                    )
                ]
            ),
            ast.statements
                .get(0)
                .expect("Failed to find statement in ast")
        );
    }
}
