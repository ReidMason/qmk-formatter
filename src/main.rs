mod lexer;
mod parser;

fn main() {
    let content = r##"[_QWERTY] = LAYOUT(
  KC_ESC  , KC_Q , KC_W , KC_E    , KC_R , KC_T ,                                                 KC_Y , KC_U    , KC_I    , KC_O   , KC_P    , KC_BSPC ,
  SFT_TAB , KC_A , KC_S , KC_D    , KC_F , KC_G ,                                                 KC_H , KC_J    , KC_K    , KC_L   , KC_SCLN , KC_QUOTE,
  KC_LCTL , KC_Z , KC_X , KC_C    , KC_V , KC_B , KC_CPYP , ADJUST  ,         FKEYS   , _______ , KC_N , KC_M    , KC_COMM , KC_DOT , KC_SLSH , KC_RSFT ,
                          KC_LALT , NAV  , SYM  , KC_ENT  , KC_LGUI ,         KC_RGUI , KC_SPC  , NAV  , _______ , _______                              
),"##.to_string();

    // let mut lexer = Lexer::new(content);
    //
    // let mut token = lexer.next_token();
    // while token.token_type 1= {
    //     println!("{}", token.unwrap());
    //     token = lexer.next_token();
    // }
}
