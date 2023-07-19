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
mod data;
mod formatter;
mod lexer;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath = args.get(1).expect("Filepath argument missing");

    let mut file = File::open(&filepath).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    let new_contents = get_formatted_file_contents(&contents);

    let mut file = File::create(&filepath).expect("Failed to open file to write");
    file.write_all(new_contents.as_bytes())
        .expect("Failed to write file");

    return;
}

fn get_formatted_file_contents(content: &str) -> String {
    // let content = get_keymap();

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
    let ast = parser.parse();

    let mut formatting = String::new();
    for statement in ast.statements {
        let (start, end, keymaps) = match statement {
            // ast::StatementEnum::KeymapStatement(x) => x,
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
            // for (i, key) in keymap.layout_statement.keys.iter().enumerate() {
            //     formatting += match key {
            //         key if "" == key.trim() => "_______",
            //         key => key,
            //     };
            //
            //     if i < keymap.layout_statement.keys.len() - 1 {
            //         formatting += ",";
            //     }
            // }
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
