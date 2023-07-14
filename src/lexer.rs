use std::fmt;

pub struct Lexer {
    content: String,
    ch: Option<char>,
    position: usize,
    read_position: usize,
}

impl Lexer {
    pub fn new(content: String) -> Self {
        let ch = content.chars().next();

        Self {
            content,
            position: 0,
            read_position: 1,
            ch,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let ch = match self.ch {
            Some(x) => x,
            None => return Token::new(TokenType::EOF, "", self.position),
        };

        let token: Token = match ch {
            '(' => Token::new(TokenType::LParen, &ch.to_string(), self.position),
            ')' => Token::new(TokenType::RParen, &ch.to_string(), self.position),
            '[' => Token::new(TokenType::LSqBrace, &ch.to_string(), self.position),
            ']' => Token::new(TokenType::RSqBrace, &ch.to_string(), self.position),
            ',' => Token::new(TokenType::Comma, &ch.to_string(), self.position),
            '=' => Token::new(TokenType::Equals, &ch.to_string(), self.position),
            _ => {
                let position = self.position;
                let identifier = self.read_identifier();

                if identifier == "LAYOUT" {
                    return Token::new(TokenType::Layout, &identifier, position);
                } else if identifier.replace("_", "").is_empty() {
                    return Token::new(TokenType::Blank, &identifier, position);
                }

                return Token::new(TokenType::Unknown, &identifier, position);
            }
        };

        self.read_char();

        return token;
    }

    fn read_identifier(&mut self) -> String {
        // TODO: Remove the unwraps here
        let mut identifier = "".to_string();
        while identifier.is_empty() || self.ch.unwrap().is_alphanumeric() || self.ch == Some('_') {
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

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    LParen,
    RParen,
    LSqBrace,
    RSqBrace,
    Equals,
    Comma,
    Layout,
    Blank,
    Unknown,
    EOF,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
    pub position: usize,
}

impl Token {
    pub fn new(token_type: TokenType, literal: &str, position: usize) -> Self {
        Self {
            token_type,
            literal: literal.to_string(),
            position,
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
        let content = r##"// 
        [_QWERTY] = LAYOUT(
  KC_ESC  , KC_Q , _____ , KC_E 
  ),"##
            .to_string();

        let mut expected_types: Vec<Token> = vec![
            Token::new(TokenType::Unknown, "/", 0),
            Token::new(TokenType::Unknown, "/", 1),
            Token::new(TokenType::LSqBrace, "[", 12),
            Token::new(TokenType::Unknown, "_QWERTY", 13),
            Token::new(TokenType::RSqBrace, "]", 20),
            Token::new(TokenType::Equals, "=", 22),
            Token::new(TokenType::Layout, "LAYOUT", 24),
            Token::new(TokenType::LParen, "(", 30),
            Token::new(TokenType::Unknown, "KC_ESC", 34),
            Token::new(TokenType::Comma, ",", 42),
            Token::new(TokenType::Unknown, "KC_Q", 44),
            Token::new(TokenType::Comma, ",", 49),
            Token::new(TokenType::Blank, "_____", 51),
            Token::new(TokenType::Comma, ",", 57),
            Token::new(TokenType::Unknown, "KC_E", 59),
            Token::new(TokenType::RParen, ")", 67),
            Token::new(TokenType::Comma, ",", 68),
        ];

        let mut lexer = Lexer::new(content);

        let mut token = lexer.next_token();
        let mut counter = 1;
        while token.token_type != TokenType::EOF {
            let expected = expected_types.remove(0);

            assert_eq!(expected, token, "Failed at token: {}", counter);

            counter += 1;
            token = lexer.next_token();
        }

        assert!(expected_types.is_empty());
    }
}
