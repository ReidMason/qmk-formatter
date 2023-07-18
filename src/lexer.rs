pub struct Lexer {
    content: Vec<u8>,
    ch: u8,
    position: usize,
    read_position: usize,
}

impl Lexer {
    pub fn new(content: &str) -> Self {
        let mut lexer = Self {
            content: content.to_string().into_bytes(),
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
            b'{' => TokenType::LBrace(self.position),
            b'}' => TokenType::RBrace(self.position),
            b'[' => TokenType::LSqBrace(self.position),
            b']' => TokenType::RSqBrace(self.position),
            b',' => TokenType::Comma(self.position),
            b'=' => TokenType::Equals(self.position),
            b'/' => {
                let position = self.position;
                if self.peek_char() == b'/' {
                    let line = self.read_to_end_of_line();
                    return TokenType::Comment(position, self.position, line);
                }

                return TokenType::Ident(self.position, self.ch.to_string());
            }
            0 => TokenType::EOF,
            _ => {
                let position = self.position;
                let identifier = self.read_identifier();

                return match identifier {
                    _ if identifier == "LAYOUT".to_string() => TokenType::Layout(position),
                    _ if identifier == "const".to_string() => TokenType::Const(position),
                    _ if identifier == "PROGMEM".to_string() => TokenType::Progmem(position),
                    _ if identifier.replace("_", "").is_empty() => TokenType::Blank(position),
                    _ => TokenType::Ident(position, identifier),
                };
            }
        };

        self.read_char();

        return token;
    }

    fn read_to_end_of_line(&mut self) -> String {
        let pos = self.position;
        while pos == self.position || self.ch != b'\n' {
            self.read_char();
        }

        String::from_utf8_lossy(&self.content[pos..self.position]).to_string()
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

    fn peek_char(&self) -> u8 {
        if self.position >= self.content.len() {
            return 0;
        }

        self.content[self.position]
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
    Ident(usize, String),
    Const(usize),
    Comment(usize, usize, String),
    Progmem(usize),
    EOF,
    LBrace(usize),
    RBrace(usize),
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_next_token() {
        let content = r##"// testing
        [_QWERTY] = LAYOUT(
  KC_ESC  , KC_Q , _____ , KC_E 
  ),
  const uint16_t PROGMEM keymaps[][MATRIX_ROWS][MATRIX_COLS] = {"##
            .to_string();

        let mut expected_types: Vec<TokenType> = vec![
            TokenType::Comment(0, 10, "// testing".to_string()),
            TokenType::LSqBrace(19),
            TokenType::Ident(20, "_QWERTY".to_string()),
            TokenType::RSqBrace(27),
            TokenType::Equals(29),
            TokenType::Layout(31),
            TokenType::LParen(37),
            TokenType::Ident(41, "KC_ESC".to_string()),
            TokenType::Comma(49),
            TokenType::Ident(51, "KC_Q".to_string()),
            TokenType::Comma(56),
            TokenType::Blank(58),
            TokenType::Comma(64),
            TokenType::Ident(66, "KC_E".to_string()),
            TokenType::RParen(74),
            TokenType::Comma(75),
            TokenType::Const(79),
            TokenType::Ident(85, "uint16_t".to_string()),
            TokenType::Progmem(94),
            TokenType::Ident(102, "keymaps".to_string()),
            TokenType::LSqBrace(109),
            TokenType::RSqBrace(110),
            TokenType::LSqBrace(111),
            TokenType::Ident(112, "MATRIX_ROWS".to_string()),
            TokenType::RSqBrace(123),
            TokenType::LSqBrace(124),
            TokenType::Ident(125, "MATRIX_COLS".to_string()),
            TokenType::RSqBrace(136),
            TokenType::Equals(138),
            TokenType::LBrace(140),
        ];

        let mut lexer = Lexer::new(&content);

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
