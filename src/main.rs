fn main() {
    let content = r##"[_QWERTY] = LAYOUT(
  KC_ESC  , KC_Q , KC_W , KC_E    , KC_R , KC_T ,                                                 KC_Y , KC_U    , KC_I    , KC_O   , KC_P    , KC_BSPC ,
  SFT_TAB , KC_A , KC_S , KC_D    , KC_F , KC_G ,                                                 KC_H , KC_J    , KC_K    , KC_L   , KC_SCLN , KC_QUOTE,
  KC_LCTL , KC_Z , KC_X , KC_C    , KC_V , KC_B , KC_CPYP , ADJUST  ,         FKEYS   , _______ , KC_N , KC_M    , KC_COMM , KC_DOT , KC_SLSH , KC_RSFT ,
                          KC_LALT , NAV  , SYM  , KC_ENT  , KC_LGUI ,         KC_RGUI , KC_SPC  , NAV  , _______ , _______                              
),"##;
    parse(content);
}

#[derive(Debug)]
enum TokenType {
    LParen,
    RParen,
    LSqBrace,
    RSqBrace,
    Equals,
    Comma,
    Unknown,
}

struct Token {
    token_type: TokenType,
    literal: String,
}

fn parse(content: &str) {
    let mut tokens: Vec<Token> = vec![];

    let mut literal = "".to_string();
    for x in content.split("") {
        if x.is_empty() {
            continue;
        }

        let token: Option<Token> = match x {
            "(" => Some(Token {
                token_type: TokenType::LParen,
                literal: x.to_string(),
            }),
            ")" => Some(Token {
                token_type: TokenType::RParen,
                literal: x.to_string(),
            }),
            "[" => Some(Token {
                token_type: TokenType::LSqBrace,
                literal: x.to_string(),
            }),
            "]" => Some(Token {
                token_type: TokenType::RSqBrace,
                literal: x.to_string(),
            }),
            "," => Some(Token {
                token_type: TokenType::Comma,
                literal: x.to_string(),
            }),
            "=" => Some(Token {
                token_type: TokenType::Equals,
                literal: x.to_string(),
            }),
            _ => None,
        };

        if let Some(token) = token {
            tokens.push(token);
            continue;
        }

        if x.trim().is_empty() && !literal.is_empty() {
            tokens.push(Token {
                token_type: TokenType::Unknown,
                literal,
            });
            literal = "".to_string();
        } else if !x.trim().is_empty() {
            literal.push_str(x);
        }
    }

    // Print the tokens
    for token in tokens {
        println!(
            "Type: '{:?}' Literal: '{}'",
            token.token_type, token.literal
        );
    }
}
