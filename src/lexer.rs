use std::fmt;

pub struct Lexer {
    content: String,
    ch: Option<char>,
    position: usize,
    read_position: usize,
}

impl Lexer {
    pub fn new(content: String) -> Self {
        Self {
            content: content.clone(),
            position: 0,
            read_position: 1,
            ch: content.chars().next(),
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        let token: Token = match self.ch {
            Some('(') => Token {
                token_type: TokenType::LParen,
                literal: self.ch?.to_string(),
            },
            Some(')') => Token {
                token_type: TokenType::RParen,
                literal: self.ch?.to_string(),
            },
            Some('[') => Token {
                token_type: TokenType::LSqBrace,
                literal: self.ch?.to_string(),
            },
            Some(']') => Token {
                token_type: TokenType::RSqBrace,
                literal: self.ch?.to_string(),
            },
            Some(',') => Token {
                token_type: TokenType::Comma,
                literal: self.ch?.to_string(),
            },
            Some('=') => Token {
                token_type: TokenType::Equals,
                literal: self.ch?.to_string(),
            },
            None => return None,
            _ => {
                let identifier = self.read_identifier();

                if identifier == "LAYOUT" {
                    return Some(Token {
                        token_type: TokenType::Layout,
                        literal: identifier,
                    });
                } else if identifier.replace("_", "").is_empty() {
                    return Some(Token {
                        token_type: TokenType::Blank,
                        literal: identifier,
                    });
                }

                return Some(Token {
                    token_type: TokenType::Unknown,
                    literal: identifier,
                });
            }
        };

        self.read_char();

        return Some(token);
    }

    fn read_identifier(&mut self) -> String {
        let mut identifier = "".to_string();
        while self.ch.unwrap().is_alphanumeric() || self.ch == Some('_') {
            identifier.push_str(&self.ch.unwrap().to_string());
            self.read_char();
        }

        identifier
    }

    fn read_char(&mut self) {
        self.ch = self.content.chars().nth(self.read_position);
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn skip_whitespace(&mut self) {
        while self.ch == Some(' ')
            || self.ch == Some('\t')
            || self.ch == Some('\n')
            || self.ch == Some('\r')
        {
            self.read_char();
        }
    }
}

#[derive(Debug, PartialEq)]
enum TokenType {
    LParen,
    RParen,
    LSqBrace,
    RSqBrace,
    Equals,
    Comma,
    Layout,
    Blank,
    Unknown,
}

pub struct Token {
    token_type: TokenType,
    literal: String,
}

impl Token {
    fn new(token_type: TokenType, literal: &str) -> Self {
        Self {
            token_type,
            literal: literal.to_string(),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(
            f,
            "Type: '{:?}' Literal: '{}'",
            self.token_type, self.literal
        )
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_next_token() {
        let content = r##"[_QWERTY] = LAYOUT(
  KC_ESC  , KC_Q , _____ , KC_E 
  ),"##
            .to_string();

        let mut expected_types: Vec<Token> = vec![
            Token::new(TokenType::LSqBrace, "["),
            Token::new(TokenType::Unknown, "_QWERTY"),
            Token::new(TokenType::RSqBrace, "]"),
            Token::new(TokenType::Equals, "="),
            Token::new(TokenType::Layout, "LAYOUT"),
            Token::new(TokenType::LParen, "("),
            Token::new(TokenType::Unknown, "KC_ESC"),
            Token::new(TokenType::Comma, ","),
            Token::new(TokenType::Unknown, "KC_Q"),
            Token::new(TokenType::Comma, ","),
            Token::new(TokenType::Blank, "_____"),
            Token::new(TokenType::Comma, ","),
            Token::new(TokenType::Unknown, "KC_E"),
            Token::new(TokenType::RParen, ")"),
            Token::new(TokenType::Comma, ","),
        ];

        let mut lexer = Lexer::new(content);

        let mut token = lexer.next_token();
        let mut counter = 1;
        while token.is_some() {
            let expected = expected_types.remove(0);
            let t = token.as_ref().unwrap();

            assert_eq!(
                expected.token_type, t.token_type,
                "Failed at token: {}",
                counter
            );
            assert_eq!(expected.literal, t.literal, "Failed at token: {}", counter);

            counter += 1;
            token = lexer.next_token();
        }

        assert!(expected_types.is_empty());
    }
}
