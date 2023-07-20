use std::{
    env,
    fs::File,
    io::{Read, Write},
};

use lexer::Lexer;
use parser::Parser;

use crate::{
    formatter::{get_keymap_format, get_keymap_string, M},
    lexer::TokenType,
};

mod ast;
mod formatter;
mod lexer;
mod parser;

type Layout = Vec<Vec<M>>;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath = args.get(1).expect("Filepath argument missing");

    let layout: Layout = vec![
        vec![
            M::K,
            M::K,
            M::K,
            M::K,
            M::K,
            M::K,
            M::B,
            M::B,
            M::B,
            M::B,
            M::B,
            M::K,
            M::K,
            M::K,
            M::K,
            M::K,
            M::K,
        ],
        vec![
            M::K,
            M::K,
            M::K,
            M::K,
            M::K,
            M::K,
            M::B,
            M::B,
            M::B,
            M::B,
            M::B,
            M::K,
            M::K,
            M::K,
            M::K,
            M::K,
            M::K,
        ],
        vec![
            M::K,
            M::K,
            M::K,
            M::K,
            M::K,
            M::K,
            M::K,
            M::K,
            M::B,
            M::K,
            M::K,
            M::K,
            M::K,
            M::K,
            M::K,
            M::K,
            M::K,
        ],
        vec![
            M::B,
            M::B,
            M::B,
            M::K,
            M::K,
            M::K,
            M::K,
            M::K,
            M::B,
            M::K,
            M::K,
            M::K,
            M::K,
            M::K,
            M::B,
            M::B,
            M::B,
        ],
    ];

    let mut file = File::open(&filepath).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    let new_contents = get_formatted_file_contents(&contents, layout);

    let mut file = File::create(&filepath).expect("Failed to open file to write");
    file.write_all(new_contents.as_bytes())
        .expect("Failed to write file");

    return;
}

fn get_formatted_file_contents(content: &str, layout: Layout) -> String {
    let lexer = Lexer::new(&content);
    let mut parser = Parser::new(lexer);

    let ast = parser.parse();

    let mut formatting = String::new();
    for statement in ast.statements {
        let (start, end, keymaps) = match statement {
            ast::StatementEnum::KeymapStatement(..) => continue,
            ast::StatementEnum::Keymaps(x, y, z) => (x, y, z),
        };

        for keymap in keymaps {
            let (display, keymap_formatted) = get_keymap_format(&keymap, layout.clone());

            let display = get_keymap_string(display);
            let keymap_formatted = get_keymap_string(keymap_formatted);
            formatting += &display;
            formatting += "\n";

            formatting += "[";
            formatting += match &keymap.token {
                TokenType::Ident(_, x) => x,
                _ => "",
            };

            formatting += "] = LAYOUT(\n";
            formatting += &keymap_formatted;

            formatting += "),";
            formatting += "\n\n";
        }

        let (first, last) = content.split_at(start + 1);
        let (_, ending) = last.split_at(end - start - 1);
        let res = format!("{}\n{}\n{}", first, formatting, ending);

        return res;
    }

    return content.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_formatting_full_code() {
        let content = r##"const thing = other;
// top comments
const uint16_t PROGMEM keymaps[][MATRIX_ROWS][MATRIX_COLS] = {
//  Comments
[_QWERTY] = LAYOUT(
 KC_ESC  , KC_Q    , KC_W    , KC_E    , KC_R    , KC_T    ,                                                   KC_Y    , KC_U    , KC_I    , KC_O    , KC_P    , KC_BSPC ,          
 SFT_TAB , KC_A    , KC_S    , KC_D    , KC_F    , KC_G    ,                                                   KC_H    , KC_J    , KC_K    , KC_L    , KC_SCLN , SFT_QOT ,          
 KC_LCTL , KC_Z    , KC_X    , KC_C    , KC_V    , KC_B    , KC_CPYP , ADJUST  ,           FKEYS   , _______ , KC_N    , KC_M    , KC_COMM , KC_DOT  , KC_SLSH , KC_RCTL ,          
                               KC_LALT , NAV     , KC_LGUI , KC_ENT  , SYM     ,           SYM     , KC_SPC  , NAV     , _______ , _______                                          
),
// More comments and things
[_NAV] = LAYOUT(
 _______ , _______ , _______ , _______ , _______ , _______ ,                                                   _______ , _______ , _______ , _______ , _______ , KC_DEL  ,          
 _______ , KC_LGUI , KC_LALT , KC_LCTL , KC_LSFT , _______ ,                                                   KC_LEFT , KC_DOWN , KC_UP   , KC_RGHT , _______ , _______ ,          
 _______ , _______ , _______ , _______ , _______ , _______ , _______ , KC_SCRL ,           _______ , _______ , _______ , _______ , _______ , _______ , _______ , _______ ,          
                               _______ , _______ , _______ , _______ , _______ ,           _______ , _______ , _______ , _______ , _______                                          
),
}
// something
const thing = other;
"##;
        let layout: Layout = vec![
            vec![
                M::K,
                M::K,
                M::K,
                M::K,
                M::K,
                M::K,
                M::B,
                M::B,
                M::B,
                M::B,
                M::B,
                M::K,
                M::K,
                M::K,
                M::K,
                M::K,
                M::K,
            ],
            vec![
                M::K,
                M::K,
                M::K,
                M::K,
                M::K,
                M::K,
                M::B,
                M::B,
                M::B,
                M::B,
                M::B,
                M::K,
                M::K,
                M::K,
                M::K,
                M::K,
                M::K,
            ],
            vec![
                M::K,
                M::K,
                M::K,
                M::K,
                M::K,
                M::K,
                M::K,
                M::K,
                M::B,
                M::K,
                M::K,
                M::K,
                M::K,
                M::K,
                M::K,
                M::K,
                M::K,
            ],
            vec![
                M::B,
                M::B,
                M::B,
                M::K,
                M::K,
                M::K,
                M::K,
                M::K,
                M::B,
                M::K,
                M::K,
                M::K,
                M::K,
                M::K,
                M::B,
                M::B,
                M::B,
            ],
        ];
        let result = get_formatted_file_contents(content, layout);

        let expected = r##"const thing = other;
// top comments
const uint16_t PROGMEM keymaps[][MATRIX_ROWS][MATRIX_COLS] = {
//    ╭─────────┬─────────┬─────────┬─────────┬─────────┬─────────╮                                                 ╭─────────┬─────────┬─────────┬─────────┬─────────┬─────────╮         
//    │ KC_ESC  │ KC_Q    │ KC_W    │ KC_E    │ KC_R    │ KC_T    │                                                 │ KC_Y    │ KC_U    │ KC_I    │ KC_O    │ KC_P    │ KC_BSPC │         
//    ├─────────┼─────────┼─────────┼─────────┼─────────┼─────────┤                                                 ├─────────┼─────────┼─────────┼─────────┼─────────┼─────────┤         
//    │ SFT_TAB │ KC_A    │ KC_S    │ KC_D    │ KC_F    │ KC_G    │                                                 │ KC_H    │ KC_J    │ KC_K    │ KC_L    │ KC_SCLN │ SFT_QOT │         
//    ├─────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┬─────────╮         ╭─────────┬─────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┤         
//    │ KC_LCTL │ KC_Z    │ KC_X    │ KC_C    │ KC_V    │ KC_B    │ KC_CPYP │ ADJUST  │         │ FKEYS   │         │ KC_N    │ KC_M    │ KC_COMM │ KC_DOT  │ KC_SLSH │ KC_RCTL │         
//    ╰─────────┴─────────┴─────────┼─────────┼─────────┼─────────┼─────────┼─────────┤         ├─────────┼─────────┼─────────┼─────────┼─────────┼─────────┴─────────┴─────────╯         
//                                  │ KC_LALT │ NAV     │ KC_LGUI │ KC_ENT  │ SYM     │         │ SYM     │ KC_SPC  │ NAV     │         │         │                                       
//                                  ╰─────────┴─────────┴─────────┴─────────┴─────────╯         ╰─────────┴─────────┴─────────┴─────────┴─────────╯                                       
[_QWERTY] = LAYOUT(
 KC_ESC  , KC_Q    , KC_W    , KC_E    , KC_R    , KC_T    ,                                                   KC_Y    , KC_U    , KC_I    , KC_O    , KC_P    , KC_BSPC ,          
 SFT_TAB , KC_A    , KC_S    , KC_D    , KC_F    , KC_G    ,                                                   KC_H    , KC_J    , KC_K    , KC_L    , KC_SCLN , SFT_QOT ,          
 KC_LCTL , KC_Z    , KC_X    , KC_C    , KC_V    , KC_B    , KC_CPYP , ADJUST  ,           FKEYS   , _______ , KC_N    , KC_M    , KC_COMM , KC_DOT  , KC_SLSH , KC_RCTL ,          
                               KC_LALT , NAV     , KC_LGUI , KC_ENT  , SYM     ,           SYM     , KC_SPC  , NAV     , _______ , _______                                          
),

//    ╭─────────┬─────────┬─────────┬─────────┬─────────┬─────────╮                                                 ╭─────────┬─────────┬─────────┬─────────┬─────────┬─────────╮         
//    │         │         │         │         │         │         │                                                 │         │         │         │         │         │ KC_DEL  │         
//    ├─────────┼─────────┼─────────┼─────────┼─────────┼─────────┤                                                 ├─────────┼─────────┼─────────┼─────────┼─────────┼─────────┤         
//    │         │ KC_LGUI │ KC_LALT │ KC_LCTL │ KC_LSFT │         │                                                 │ KC_LEFT │ KC_DOWN │ KC_UP   │ KC_RGHT │         │         │         
//    ├─────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┬─────────╮         ╭─────────┬─────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┤         
//    │         │         │         │         │         │         │         │ KC_SCRL │         │         │         │         │         │         │         │         │         │         
//    ╰─────────┴─────────┴─────────┼─────────┼─────────┼─────────┼─────────┼─────────┤         ├─────────┼─────────┼─────────┼─────────┼─────────┼─────────┴─────────┴─────────╯         
//                                  │         │         │         │         │         │         │         │         │         │         │         │                                       
//                                  ╰─────────┴─────────┴─────────┴─────────┴─────────╯         ╰─────────┴─────────┴─────────┴─────────┴─────────╯                                       
[_NAV] = LAYOUT(
 _______ , _______ , _______ , _______ , _______ , _______ ,                                                   _______ , _______ , _______ , _______ , _______ , KC_DEL  ,          
 _______ , KC_LGUI , KC_LALT , KC_LCTL , KC_LSFT , _______ ,                                                   KC_LEFT , KC_DOWN , KC_UP   , KC_RGHT , _______ , _______ ,          
 _______ , _______ , _______ , _______ , _______ , _______ , _______ , KC_SCRL ,           _______ , _______ , _______ , _______ , _______ , _______ , _______ , _______ ,          
                               _______ , _______ , _______ , _______ , _______ ,           _______ , _______ , _______ , _______ , _______                                          
),
}

// something
const thing = other;"##;

        assert_eq!(expected, result)
    }

    #[test]
    fn test_layout_with_too_little_keys() {
        let content = r##"const thing = other;
// top comments
const uint16_t PROGMEM keymaps[][MATRIX_ROWS][MATRIX_COLS] = {
//  Comments
[_QWERTY] = LAYOUT(
 KC_ESC  , KC_Q    , KC_W    , KC_E    , KC_R    , KC_T
),
}
// something
const thing = other;
"##;
        let layout: Layout = vec![vec![M::K, M::K, M::K]];
        let result = get_formatted_file_contents(content, layout);

        assert_eq!(content, result)
    }

    #[test]
    fn test_layout_with_too_many_keys() {
        let content = r##"const thing = other;
// top comments
const uint16_t PROGMEM keymaps[][MATRIX_ROWS][MATRIX_COLS] = {
//  Comments
[_QWERTY] = LAYOUT(
 KC_ESC  , KC_Q    , KC_W    , KC_E    , KC_R    , KC_T
),
}
// something
const thing = other;
"##;
        let layout: Layout = vec![vec![M::K, M::K, M::K, M::K, M::K, M::K, M::K]];
        let result = get_formatted_file_contents(content, layout);

        assert_eq!(content, result)
    }
}
