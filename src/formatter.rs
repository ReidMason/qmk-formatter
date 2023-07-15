use crate::ast::KeymapStatement;

#[derive(Debug, PartialEq, Clone)]
pub enum Border {
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
    Horizontal,
    Vertical,
    TopT,
    BottomT,
    Newline,
    LineStart,
    Space,
    Key(String),
}

// TODO: Remove the pub
pub fn get_border_name(border: &Border) -> String {
    match border {
        Border::TopLeft => "╭".to_string(),
        Border::TopRight => "╮".to_string(),
        Border::BottomRight => "╯".to_string(),
        Border::BottomLeft => "╰".to_string(),
        Border::Horizontal => "─".to_string(),
        Border::Vertical => "│".to_string(),
        Border::TopT => "┬".to_string(),
        Border::BottomT => "┴".to_string(),
        Border::Newline => "\n".to_string(),
        Border::LineStart => "//    ".to_string(),
        Border::Space => " ".to_string(),
        Border::Key(x) => x.to_string(),
    }
}

pub enum Mark {
    Key,
    Blank,
}

type Layout = Vec<Vec<Mark>>;

pub fn get_keymap_format(keymap: KeymapStatement, layout: Layout) -> Vec<Border> {
    let mut output: Vec<Border> = vec![Border::LineStart];

    let keys = keymap.layout_statement.keys;
    let max_width = keys
        .iter()
        .map(|x| x.len())
        .max()
        .expect("Can't find longest keycode");

    // Create top row
    for (i, col) in layout[0].iter().enumerate() {
        let mut prev: &Mark = &Mark::Blank;
        if i > 0 {
            prev = match layout[0].get(i - 1) {
                Some(x) => x,
                None => &Mark::Blank,
            };
        }

        let next = match layout[0].get(i + 1) {
            Some(x) => x,
            None => &Mark::Blank,
        };

        match (prev, col, next) {
            (Mark::Key, Mark::Key, Mark::Key) => output.push(Border::TopT),
            (Mark::Key, Mark::Key, Mark::Blank) => output.push(Border::TopT),
            (Mark::Key, Mark::Blank, Mark::Key) => output.push(Border::TopRight),
            (Mark::Key, Mark::Blank, Mark::Blank) => output.push(Border::TopRight),
            (Mark::Blank, Mark::Key, Mark::Key) => output.push(Border::TopLeft),
            (Mark::Blank, Mark::Key, Mark::Blank) => output.push(Border::TopLeft),
            (Mark::Blank, Mark::Blank, Mark::Key) => output.push(Border::Space),
            (Mark::Blank, Mark::Blank, Mark::Blank) => output.push(Border::Space),
        };

        // Get filler character
        let filler = match col {
            Mark::Key => Border::Horizontal,
            Mark::Blank => Border::Space,
        };

        for _ in 0..max_width + 2 {
            output.push(filler.clone());
        }

        // Check if this was the last element
        if i == layout[0].len() - 1 {
            match prev {
                Mark::Key => output.push(Border::TopRight),
                Mark::Blank => {}
            };
            output.push(Border::Newline);
        }
    }

    // Process keys
    let mut count = 0;
    for row in layout.iter() {
        output.push(Border::LineStart);

        for (i, col) in row.iter().enumerate() {
            let mut prev: &Mark = &Mark::Blank;
            if i > 0 {
                prev = match row.get(i - 1) {
                    Some(x) => x,
                    None => &Mark::Blank,
                };
            }

            let next = match row.get(i + 1) {
                Some(x) => x,
                None => &Mark::Blank,
            };

            match (prev, col) {
                (Mark::Key, Mark::Key) => output.push(Border::Vertical),
                (Mark::Key, Mark::Blank) => output.push(Border::Vertical),
                (Mark::Blank, Mark::Key) => output.push(Border::Vertical),
                (Mark::Blank, Mark::Blank) => output.push(Border::Space),
            };

            output.push(Border::Space);

            let key = match col {
                Mark::Key => {
                    let key = &keys[count];
                    output.push(Border::Key(key.to_string()));
                    count += 1;
                    key
                }
                Mark::Blank => "",
            };

            for _ in key.len()..max_width {
                output.push(Border::Space);
            }

            output.push(Border::Space);

            // Check if this was the last element
            if i == row.len() - 1 {
                match (col, prev) {
                    (Mark::Key, Mark::Key) => output.push(Border::Vertical),
                    (Mark::Key, Mark::Blank) => output.push(Border::Vertical),
                    (Mark::Blank, Mark::Key) => output.push(Border::Vertical),
                    (Mark::Blank, Mark::Blank) => {}
                };
            };
        }

        output.push(Border::Newline);
    }

    // Create buttom row
    output.push(Border::LineStart);
    for (i, col) in layout[layout.len() - 1].iter().enumerate() {
        let mut prev: &Mark = &Mark::Blank;
        if i > 0 {
            prev = match layout[layout.len() - 1].get(i - 1) {
                Some(x) => x,
                None => &Mark::Blank,
            };
        }

        let next = match layout[layout.len() - 1].get(i + 1) {
            Some(x) => x,
            None => &Mark::Blank,
        };

        match (prev, col, next) {
            (Mark::Key, Mark::Key, Mark::Key) => output.push(Border::BottomT),
            (Mark::Key, Mark::Key, Mark::Blank) => output.push(Border::BottomT),
            (Mark::Key, Mark::Blank, Mark::Key) => output.push(Border::BottomRight),
            (Mark::Key, Mark::Blank, Mark::Blank) => output.push(Border::BottomRight),
            (Mark::Blank, Mark::Key, Mark::Key) => output.push(Border::BottomLeft),
            (Mark::Blank, Mark::Key, Mark::Blank) => output.push(Border::BottomLeft),
            (Mark::Blank, Mark::Blank, Mark::Key) => output.push(Border::Space),
            (Mark::Blank, Mark::Blank, Mark::Blank) => output.push(Border::Space),
        };

        // Get filler character
        let filler = match col {
            Mark::Key => Border::Horizontal,
            Mark::Blank => Border::Space,
        };

        for _ in 0..max_width + 2 {
            output.push(filler.clone());
        }

        // Check if this was the last element
        if i == layout[layout.len() - 1].len() - 1 {
            match prev {
                Mark::Key => output.push(Border::TopT),
                Mark::Blank => {}
            };
            output.push(Border::Newline);
        }
    }

    // let output: String = output.iter().map(|x| get_border_name(x)).collect();

    output
}

#[cfg(test)]
mod tests {
    use crate::{ast::LayoutStatement, lexer::TokenType};

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_format() {
        let keymap = KeymapStatement {
            token: TokenType::Ident(4932, "_QWERTY".to_string()),
            layout_statement: LayoutStatement {
                token: TokenType::Layout(4943),
                keys: vec![
                    "KC_ESC".to_string(),
                    "KC_Q".to_string(),
                    "KC_W".to_string(),
                    "KC_E".to_string(),
                    "KC_R".to_string(),
                    "KC_T".to_string(),
                    "KC_Y".to_string(),
                    "KC_U".to_string(),
                    "KC_I".to_string(),
                    "KC_O".to_string(),
                    "KC_P".to_string(),
                    "KC_BSPC".to_string(),
                    "SFT_TAB".to_string(),
                    "KC_A".to_string(),
                    "KC_S".to_string(),
                    "KC_D".to_string(),
                    "KC_F".to_string(),
                    "KC_G".to_string(),
                    "KC_H".to_string(),
                    "KC_J".to_string(),
                    "KC_K".to_string(),
                    "KC_L".to_string(),
                    "KC_SCLN".to_string(),
                    "KC_QUOTE".to_string(),
                    "KC_LCTL".to_string(),
                    "KC_Z".to_string(),
                    "KC_X".to_string(),
                    "KC_C".to_string(),
                    "KC_V".to_string(),
                    "KC_B".to_string(),
                    "KC_CPYP".to_string(),
                    "ADJUST".to_string(),
                    "FKEYS".to_string(),
                    "".to_string(),
                    "KC_N".to_string(),
                    "KC_M".to_string(),
                    "KC_COMM".to_string(),
                    "KC_DOT".to_string(),
                    "KC_SLSH".to_string(),
                    "KC_RSFT".to_string(),
                    "KC_LALT".to_string(),
                    "NAV".to_string(),
                    "SYM".to_string(),
                    "KC_ENT".to_string(),
                    "KC_LGUI".to_string(),
                    "KC_RGUI".to_string(),
                    "KC_SPC".to_string(),
                    "NAV".to_string(),
                    "".to_string(),
                    "".to_string(),
                ],
            },
        };

        let layout: Vec<Vec<Mark>> = vec![
            vec![
                Mark::Key,
                Mark::Key,
                Mark::Key,
                Mark::Key,
                Mark::Key,
                Mark::Key,
                Mark::Blank,
                Mark::Blank,
                Mark::Blank,
                Mark::Blank,
                Mark::Blank,
                Mark::Key,
                Mark::Key,
                Mark::Key,
                Mark::Key,
                Mark::Key,
                Mark::Key,
            ],
            vec![
                Mark::Key,
                Mark::Key,
                Mark::Key,
                Mark::Key,
                Mark::Key,
                Mark::Key,
                Mark::Blank,
                Mark::Blank,
                Mark::Blank,
                Mark::Blank,
                Mark::Blank,
                Mark::Key,
                Mark::Key,
                Mark::Key,
                Mark::Key,
                Mark::Key,
                Mark::Key,
            ],
            vec![
                Mark::Key,
                Mark::Key,
                Mark::Key,
                Mark::Key,
                Mark::Key,
                Mark::Key,
                Mark::Blank,
                Mark::Blank,
                Mark::Blank,
                Mark::Blank,
                Mark::Blank,
                Mark::Key,
                Mark::Key,
                Mark::Key,
                Mark::Key,
                Mark::Key,
                Mark::Key,
            ],
            vec![
                Mark::Blank,
                Mark::Blank,
                Mark::Blank,
                Mark::Key,
                Mark::Key,
                Mark::Key,
                Mark::Key,
                Mark::Key,
                Mark::Blank,
                Mark::Key,
                Mark::Key,
                Mark::Key,
                Mark::Key,
                Mark::Blank,
                Mark::Blank,
                Mark::Blank,
            ],
        ];
        let result = get_keymap_format(keymap, layout);
        // let expected = "//    ╭────────┬────────┬────╮\n".to_string() + //end
        //                "//    │ KC_ESC │ KC_Q   │     \n" + //end
        //                "//    ╰────────┴────────┴────╯\n";
        let expected: Vec<Border> = vec![
            Border::LineStart,
            Border::TopLeft,
            Border::Horizontal,
            Border::Horizontal,
            Border::Horizontal,
            Border::Horizontal,
            Border::Horizontal,
            Border::Horizontal,
            Border::Horizontal,
            Border::Horizontal,
            Border::Horizontal,
            Border::TopT,
            Border::Horizontal,
            Border::Horizontal,
            Border::Horizontal,
            Border::Horizontal,
            Border::Horizontal,
            Border::Horizontal,
            Border::Horizontal,
            Border::Horizontal,
            Border::Horizontal,
            Border::TopT,
        ];

        let output: String = result.iter().map(|x| get_border_name(x)).collect();
        println!("{}", output);

        // assert_eq!(expected, result);
    }
}
