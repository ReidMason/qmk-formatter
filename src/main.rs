use data::get_keymap;
use lexer::Lexer;
use parser::Parser;

use crate::{
    formatter::{get_keymap_format, get_keymap_string, M},
    lexer::TokenType,
};

mod ast;
mod data;
mod formatter;
mod lexer;
mod parser;

fn main() {
    let content = get_keymap();

    let lexer = Lexer::new(&content);
    let mut parser = Parser::new(lexer);

    let layout: Vec<Vec<M>> = vec![
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
            M::B,
            M::B,
            M::B,
        ],
    ];

    let ast = parser.parse();

    let mut formatting = String::new();
    for statement in ast.statements {
        let (start, end, keymaps) = match statement {
            // ast::StatementEnum::KeymapStatement(x) => x,
            ast::StatementEnum::KeymapStatement(..) => continue,
            ast::StatementEnum::Keymaps(x, y, z) => (x, y, z),
        };

        for keymap in keymaps {
            let result = get_keymap_format(&keymap, layout.clone());

            let display = get_keymap_string(result);
            formatting += &display;
            formatting += "\n";

            formatting += "[";
            formatting += match &keymap.token {
                TokenType::Ident(_, x) => x,
                _ => "",
            };

            formatting += "] = LAYOUT(\n";
            for key in &keymap.layout_statement.keys {
                formatting += match key {
                    key if "" == key.trim() => "______",
                    key => key,
                };
                formatting += ",";
            }

            formatting += "),";
            formatting += "\n\n";
        }

        let (first, last) = content.split_at(start + 1);
        let (_, ending) = last.split_at(end - start - 1);
        let res = format!("{}\n{}\n{}", first, formatting, ending);

        println!("{}", res);
        // println!("Start: {} End: {}", start, end);
        break;
    }
}
