pub struct Lexer {
    content: Vec<u8>,
    ch: u8,
    position: usize,
    read_position: usize,
}

impl Lexer {
    pub fn new(content: String) -> Self {
        let mut lexer = Self {
            content: content.into_bytes(),
            position: 0,
            read_position: 0,
            ch: 0,
        };

        lexer.read_char();

        lexer
    }

    pub fn next_token(&mut self) -> TokenType {
        self.skip_whitespace();

        let token: TokenType = match self.ch {
            b'(' => TokenType::LParen(self.position),
            b')' => TokenType::RParen(self.position),
            b'[' => TokenType::LSqBrace(self.position),
            b']' => TokenType::RSqBrace(self.position),
            b',' => TokenType::Comma(self.position),
            b'=' => TokenType::Equals(self.position),
            0 => TokenType::EOF,
            _ => {
                let position = self.position;
                let identifier = self.read_identifier();

                if identifier == "LAYOUT" {
                    return TokenType::Layout(position);
                } else if identifier.replace("_", "").is_empty() {
                    return TokenType::Blank(position);
                }

                return TokenType::Unknown(position, identifier);
            }
        };

        self.read_char();

        return token;
    }

    fn read_identifier(&mut self) -> String {
        let pos = self.position;
        while pos == self.position || self.ch.is_ascii_alphanumeric() || self.ch == b'_' {
            self.read_char();
        }

        String::from_utf8_lossy(&self.content[pos..self.position]).to_string()
    }

    fn read_char(&mut self) {
        self.ch = *self.content.get(self.read_position).unwrap_or(&0);

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn skip_whitespace(&mut self) {
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
            self.read_char();
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    LParen(usize),
    RParen(usize),
    LSqBrace(usize),
    RSqBrace(usize),
    Equals(usize),
    Comma(usize),
    Layout(usize),
    Blank(usize),
    Unknown(usize, String),
    EOF,
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

        let mut expected_types: Vec<TokenType> = vec![
            TokenType::Unknown(0, "/".to_string()),
            TokenType::Unknown(1, "/".to_string()),
            TokenType::LSqBrace(12),
            TokenType::Unknown(13, "_QWERTY".to_string()),
            TokenType::RSqBrace(20),
            TokenType::Equals(22),
            TokenType::Layout(24),
            TokenType::LParen(30),
            TokenType::Unknown(34, "KC_ESC".to_string()),
            TokenType::Comma(42),
            TokenType::Unknown(44, "KC_Q".to_string()),
            TokenType::Comma(49),
            TokenType::Blank(51),
            TokenType::Comma(57),
            TokenType::Unknown(59, "KC_E".to_string()),
            TokenType::RParen(67),
            TokenType::Comma(68),
        ];

        let mut lexer = Lexer::new(content);

        let mut token = lexer.next_token();
        let mut counter = 1;
        while token != TokenType::EOF {
            let expected = expected_types.remove(0);

            assert_eq!(expected, token, "Failed at token: {}", counter);

            counter += 1;
            token = lexer.next_token();
        }

        assert!(expected_types.is_empty());
    }
}
