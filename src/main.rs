use std::{
    fs::File,
    io::{Read, Write},
    process::ExitCode,
};

use clap::Parser as ClapParser;
use formatter::Layout;
use formatter::Mark::*;
use lexer::Lexer;
use parser::Parser;

use crate::{
    formatter::{get_keymap_format, get_keymap_string, Mark},
    lexer::TokenType,
};

mod ast;
mod formatter;
mod lexer;
mod parser;

/// A command line formatter for qmk keymap files
#[derive(ClapParser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Filepath to the target file
    filepath: String,

    /// Force formatting regardless of filename
    #[arg(short, long, default_value_t = false)]
    force: bool,
}

fn main() -> ExitCode {
    let args = Args::parse();

    if !validate_args(&args) {
        return ExitCode::FAILURE;
    }

    let layout = get_layout();
    match format_file(&args.filepath, layout) {
        Ok(_) => ExitCode::SUCCESS,
        Err(_) => ExitCode::FAILURE,
    }
}

fn validate_args(args: &Args) -> bool {
    if !args.force && !args.filepath.ends_with("keymap.c") {
        println!("Provided filepath isn't a 'keymap.c' file");
        return false;
    }

    return true;
}

fn format_file(filepath: &str, layout: Layout) -> Result<(), ()> {
    let contents = read_file(filepath);
    let new_contents = get_formatted_file_contents(&contents, layout);
    write_file(filepath, &new_contents);

    return Ok(());
}

fn read_file(filepath: &str) -> String {
    let mut file = File::open(&filepath).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");
    contents
}

fn write_file(filepath: &str, content: &str) {
    let mut file = File::create(&filepath).expect("Failed to open file to write");
    file.write_all(content.as_bytes())
        .expect("Failed to write file");
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

        let layout_keys = layout
            .iter()
            .flatten()
            .filter(|x| match x {
                Mark::K => true,
                Mark::B => false,
            })
            .count();
        for keymap in keymaps {
            // Keymap has the wrong number of keys
            let keymap_keys = keymap.layout_statement.keys.len();
            if keymap_keys != layout_keys {
                println!(
                    "Keymap has {} keys layout expected {} keys",
                    keymap_keys, layout_keys
                );
                return content.to_string();
            }

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
        let res = format!("{}\n{}\n{}", first, formatting.trim(), ending);

        return res;
    }

    return content.to_string();
}

fn get_layout() -> Layout {
    return vec![
        vec![K, K, K, K, K, K, B, B, B, B, B, K, K, K, K, K, K],
        vec![K, K, K, K, K, K, B, B, B, B, B, K, K, K, K, K, K],
        vec![K, K, K, K, K, K, K, K, B, K, K, K, K, K, K, K, K],
        vec![B, B, B, K, K, K, K, K, B, K, K, K, K, K, B, B, B],
    ];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_validation_invalid_filename() {
        let args = Args {
            filepath: "/home/path/invalid.c".to_string(),
            force: false,
        };
        let result = validate_args(&args);

        assert!(!result);
    }

    #[test]
    fn test_args_validation_invalid_filename_force() {
        let args = Args {
            filepath: "/home/path/invalid.c".to_string(),
            force: true,
        };
        let result = validate_args(&args);

        assert!(result);
    }

    #[test]
    fn test_formatting_full_code() {
        let content = r##"const thing = other;
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
const thing = other;
"##;
        let layout: Layout = vec![
            vec![K, K, K, K, K, K, B, B, B, B, B, K, K, K, K, K, K],
            vec![K, K, K, K, K, K, B, B, B, B, B, K, K, K, K, K, K],
            vec![K, K, K, K, K, K, K, K, B, K, K, K, K, K, K, K, K],
            vec![B, B, B, K, K, K, K, K, B, K, K, K, K, K, B, B, B],
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

        println!("{}", result);
        for (i, (ex, res)) in expected.chars().zip(result.chars()).enumerate() {
            assert_eq!(ex, res, "Char index: {}", i);
        }
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
        let layout: Layout = vec![vec![Mark::K, Mark::K, Mark::K]];
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
        let layout: Layout = vec![vec![K, K, K, K, K, K, K]];
        let result = get_formatted_file_contents(content, layout);

        assert_eq!(content, result)
    }
}
