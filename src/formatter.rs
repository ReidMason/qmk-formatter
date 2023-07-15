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
    LeftT,
    RightT,
    Plus,
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
        Border::LeftT => "├".to_string(),
        Border::RightT => "┤".to_string(),
        Border::Plus => "┼".to_string(),
        Border::Newline => "\n".to_string(),
        Border::LineStart => "//    ".to_string(),
        Border::Space => " ".to_string(),
        Border::Key(x) => x.to_string(),
    }
}

#[derive(Clone)]
pub enum M {
    K,
    B,
}

type Layout = Vec<Vec<M>>;

pub fn get_keymap_format(keymap: KeymapStatement, layout: Layout) -> Vec<Border> {
    // Add an extra blank column on the end to make formatting easier
    let mut layout = layout.clone();
    for row in layout.iter_mut() {
        row.push(M::B)
    }

    let mut output: Vec<Border> = vec![Border::LineStart];

    let keys = keymap.layout_statement.keys;
    let max_width = keys
        .iter()
        .map(|x| x.len())
        .max()
        .expect("Can't find longest keycode");

    // Create top row
    for (i, col) in layout[0].iter().enumerate() {
        let mut prev: &M = &M::B;
        if i > 0 {
            prev = match layout[0].get(i - 1) {
                Some(x) => x,
                None => &M::B,
            };
        }

        let next = match layout[0].get(i + 1) {
            Some(x) => x,
            None => &M::B,
        };

        match (prev, col, next) {
            (M::K, M::K, M::K) => output.push(Border::TopT),
            (M::K, M::K, M::B) => output.push(Border::TopT),
            (M::K, M::B, M::K) => output.push(Border::TopRight),
            (M::K, M::B, M::B) => output.push(Border::TopRight),
            (M::B, M::K, M::K) => output.push(Border::TopLeft),
            (M::B, M::K, M::B) => output.push(Border::TopLeft),
            (M::B, M::B, M::K) => output.push(Border::Space),
            (M::B, M::B, M::B) => output.push(Border::Space),
        };

        // Get filler character
        let filler = match col {
            M::K => Border::Horizontal,
            M::B => Border::Space,
        };

        for _ in 0..max_width + 2 {
            output.push(filler.clone());
        }
    }

    // Process keys
    let mut count = 0;
    for (i, row) in layout.iter().enumerate() {
        output.push(Border::LineStart);

        for (j, col) in row.iter().enumerate() {
            let mut prev: &M = &M::B;
            if j > 0 {
                prev = match row.get(j - 1) {
                    Some(x) => x,
                    None => &M::B,
                };
            }

            // let next = match row.get(j + 1) {
            //     Some(x) => x,
            //     None => &M::B,
            // };

            match (prev, col) {
                (M::K, M::K) => output.push(Border::Vertical),
                (M::K, M::B) => output.push(Border::Vertical),
                (M::B, M::K) => output.push(Border::Vertical),
                (M::B, M::B) => output.push(Border::Space),
            };

            output.push(Border::Space);

            let key = match col {
                M::K => {
                    let key = &keys[count];
                    output.push(Border::Key(key.to_string()));
                    count += 1;
                    key
                }
                M::B => "",
            };

            for _ in key.len()..max_width {
                output.push(Border::Space);
            }

            output.push(Border::Space);
        }

        output.push(Border::Newline);

        let is_last_row = i < layout.len() - 1;
        if is_last_row {
            // Add horizontal divider
            output.push(Border::LineStart);
            for (j, col) in row.iter().enumerate() {
                let mut first: &M = &M::B;
                if i > 0 && j > 0 {
                    first = match layout.get(i - 1) {
                        Some(x) => match x.get(j - 1) {
                            Some(x) => x,
                            None => &M::B,
                        },
                        None => &M::B,
                    };
                }

                let mut second: &M = &M::B;
                if i > 0 {
                    second = match layout.get(i - 1) {
                        Some(x) => match x.get(j) {
                            Some(x) => x,
                            None => &M::B,
                        },
                        None => &M::B,
                    };
                }

                let mut third: &M = &M::B;
                if i > 0 {
                    third = match layout.get(i - 1) {
                        Some(x) => match x.get(j + 1) {
                            Some(x) => x,
                            None => &M::B,
                        },
                        None => &M::B,
                    };
                }

                let mut fourth: &M = &M::B;
                if j > 0 {
                    fourth = match row.get(j - 1) {
                        Some(x) => x,
                        None => &M::B,
                    };
                }

                let sixth = match row.get(j + 1) {
                    Some(x) => x,
                    None => &M::B,
                };
                let mut seventh: &M = &M::B;
                if j > 0 {
                    seventh = match layout.get(i + 1) {
                        Some(x) => match x.get(j - 1) {
                            Some(x) => x,
                            None => &M::B,
                        },
                        None => &M::B,
                    };
                }

                let eighth = match layout.get(i + 1) {
                    Some(x) => match x.get(j) {
                        Some(x) => x,
                        None => &M::B,
                    },
                    None => &M::B,
                };

                let ninth = match layout.get(i + 1) {
                    Some(x) => match x.get(j + 1) {
                        Some(x) => x,
                        None => &M::B,
                    },
                    None => &M::B,
                };

                match (
                    first, second, third, fourth, col, sixth, seventh, eighth, ninth,
                ) {
                    (_, _, _, M::B, M::K, _, M::B, M::B, _) => output.push(Border::BottomLeft),
                    (_, _, _, M::K, M::B, _, M::B, M::B, _) => output.push(Border::BottomRight),
                    (_, _, _, M::K, M::B, _, M::K, M::B, _) => output.push(Border::RightT),
                    (_, _, _, M::B, M::K, _, M::B, M::K, _) => output.push(Border::LeftT),
                    (_, _, _, M::K, M::K, _, M::B, M::B, _) => output.push(Border::BottomT),
                    (_, _, _, M::B, M::B, _, M::K, M::K, _) => output.push(Border::TopT),
                    (_, _, _, M::B, M::B, _, M::K, M::B, _) => output.push(Border::TopRight),
                    (_, _, _, M::B, M::B, _, M::B, M::K, _) => output.push(Border::TopLeft),
                    (_, _, _, M::K, _, _, _, _, _) => output.push(Border::Plus),
                    (_, _, _, _, M::K, _, _, _, _) => output.push(Border::Plus),
                    _ => output.push(Border::Space),
                };

                // Get filler character
                let filler = match (col, eighth) {
                    (M::K, M::K) => Border::Horizontal,
                    (M::K, M::B) => Border::Horizontal,
                    (M::B, M::K) => Border::Horizontal,
                    (M::B, M::B) => Border::Space,
                };

                for _ in 0..max_width + 2 {
                    output.push(filler.clone());
                }
            }
            output.push(Border::Newline);
        }
    }

    // Create buttom row
    output.push(Border::LineStart);
    for (i, col) in layout[layout.len() - 1].iter().enumerate() {
        let mut prev: &M = &M::B;
        if i > 0 {
            prev = match layout[layout.len() - 1].get(i - 1) {
                Some(x) => x,
                None => &M::B,
            };
        }

        let next = match layout[layout.len() - 1].get(i + 1) {
            Some(x) => x,
            None => &M::B,
        };

        match (prev, col, next) {
            (M::K, M::K, M::K) => output.push(Border::BottomT),
            (M::K, M::K, M::B) => output.push(Border::BottomT),
            (M::K, M::B, M::K) => output.push(Border::BottomRight),
            (M::K, M::B, M::B) => output.push(Border::BottomRight),
            (M::B, M::K, M::K) => output.push(Border::BottomLeft),
            (M::B, M::K, M::B) => output.push(Border::BottomLeft),
            (M::B, M::B, M::K) => output.push(Border::Space),
            (M::B, M::B, M::B) => output.push(Border::Space),
        };

        // Get filler character
        let filler = match col {
            M::K => Border::Horizontal,
            M::B => Border::Space,
        };

        for _ in 0..max_width + 2 {
            output.push(filler.clone());
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
                M::B,
                M::B,
                M::B,
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
