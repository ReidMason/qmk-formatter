use crate::ast::KeymapStatement;

#[derive(Debug, PartialEq, Clone)]
pub enum Element {
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

pub fn get_border_name(border: &Element) -> String {
    match border {
        Element::TopLeft => "╭".to_string(),
        Element::TopRight => "╮".to_string(),
        Element::BottomRight => "╯".to_string(),
        Element::BottomLeft => "╰".to_string(),
        Element::Horizontal => "─".to_string(),
        Element::Vertical => "│".to_string(),
        Element::TopT => "┬".to_string(),
        Element::BottomT => "┴".to_string(),
        Element::LeftT => "├".to_string(),
        Element::RightT => "┤".to_string(),
        Element::Plus => "┼".to_string(),
        Element::Newline => "\n".to_string(),
        Element::LineStart => "//    ".to_string(),
        Element::Space => " ".to_string(),
        Element::Key(x) => x.to_string(),
    }
}

#[derive(Clone)]
pub enum M {
    K,
    B,
}

type Layout = Vec<Vec<M>>;

pub fn get_keymap_format(keymap: &KeymapStatement, layout: Layout) -> (Vec<Element>, Vec<Element>) {
    // Add an extra blank column on the end to make formatting easier
    let mut layout = layout.clone();
    for row in layout.iter_mut() {
        row.push(M::B)
    }

    let mut output: Vec<Element> = vec![Element::LineStart];
    let mut output2: Vec<Element> = vec![];

    let keys = &keymap.layout_statement.keys;
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
            (M::K, M::K, M::K) => output.push(Element::TopT),
            (M::K, M::K, M::B) => output.push(Element::TopT),
            (M::K, M::B, M::K) => output.push(Element::TopRight),
            (M::K, M::B, M::B) => output.push(Element::TopRight),
            (M::B, M::K, M::K) => output.push(Element::TopLeft),
            (M::B, M::K, M::B) => output.push(Element::TopLeft),
            (M::B, M::B, M::K) => output.push(Element::Space),
            (M::B, M::B, M::B) => output.push(Element::Space),
        };

        // Get filler character
        let filler = match col {
            M::K => Element::Horizontal,
            M::B => Element::Space,
        };

        for _ in 0..max_width + 2 {
            output.push(filler.clone());
        }
    }
    output.push(Element::Newline);

    // Process keys
    let mut count = 0;
    for (i, row) in layout.iter().enumerate() {
        output.push(Element::LineStart);

        for (j, col) in row.iter().enumerate() {
            let mut prev: &M = &M::B;
            if j > 0 {
                prev = match row.get(j - 1) {
                    Some(x) => x,
                    None => &M::B,
                };
            }

            match (prev, col) {
                (M::K, M::K) => output.push(Element::Vertical),
                (M::K, M::B) => output.push(Element::Vertical),
                (M::B, M::K) => output.push(Element::Vertical),
                (M::B, M::B) => output.push(Element::Space),
            };

            output.push(Element::Space);
            output2.push(Element::Space);

            let key = match col {
                M::K => {
                    let key = &keys[count];
                    output.push(Element::Key(key.to_string()));
                    key
                }
                M::B => "",
            };

            for _ in key.len()..max_width {
                output.push(Element::Space);
            }

            let key = match col {
                M::K => {
                    let key = &keys[count];
                    count += 1;

                    if !key.is_empty() {
                        output2.push(Element::Key(key.to_string()));
                        key
                    } else {
                        output2.push(Element::Key("_______".to_string()));
                        "_______"
                    }
                }
                M::B => "",
            };

            for _ in key.len()..max_width {
                output2.push(Element::Space);
            }

            output.push(Element::Space);
            output2.push(Element::Space);
            match col {
                M::K => {
                    if count < keys.len() {
                        output2.push(Element::Key(",".to_string()));
                    } else {
                        output2.push(Element::Space);
                    }
                }
                _ => output2.push(Element::Space),
            }
        }

        output.push(Element::Newline);
        output2.push(Element::Newline);

        let is_last_row = i < layout.len() - 1;
        if is_last_row {
            // Add horizontal divider
            output.push(Element::LineStart);
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
                    (_, _, _, M::B, M::K, _, M::B, M::B, _) => output.push(Element::BottomLeft),
                    (_, _, _, M::K, M::B, _, M::B, M::B, _) => output.push(Element::BottomRight),
                    (_, _, _, M::K, M::B, _, M::K, M::B, _) => output.push(Element::RightT),
                    (_, _, _, M::B, M::K, _, M::B, M::K, _) => output.push(Element::LeftT),
                    (_, _, _, M::K, M::K, _, M::B, M::B, _) => output.push(Element::BottomT),
                    (_, _, _, M::B, M::B, _, M::K, M::K, _) => output.push(Element::TopT),
                    (_, _, _, M::B, M::B, _, M::K, M::B, _) => output.push(Element::TopRight),
                    (_, _, _, M::B, M::B, _, M::B, M::K, _) => output.push(Element::TopLeft),
                    (_, _, _, M::K, _, _, _, _, _) => output.push(Element::Plus),
                    (_, _, _, _, M::K, _, _, _, _) => output.push(Element::Plus),
                    _ => output.push(Element::Space),
                };

                // Get filler character
                let filler = match (col, eighth) {
                    (M::K, M::K) => Element::Horizontal,
                    (M::K, M::B) => Element::Horizontal,
                    (M::B, M::K) => Element::Horizontal,
                    (M::B, M::B) => Element::Space,
                };

                for _ in 0..max_width + 2 {
                    output.push(filler.clone());
                }
            }
            output.push(Element::Newline);
        }
    }

    // Create buttom row
    output.push(Element::LineStart);
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
            (M::K, M::K, M::K) => output.push(Element::BottomT),
            (M::K, M::K, M::B) => output.push(Element::BottomT),
            (M::K, M::B, M::K) => output.push(Element::BottomRight),
            (M::K, M::B, M::B) => output.push(Element::BottomRight),
            (M::B, M::K, M::K) => output.push(Element::BottomLeft),
            (M::B, M::K, M::B) => output.push(Element::BottomLeft),
            (M::B, M::B, M::K) => output.push(Element::Space),
            (M::B, M::B, M::B) => output.push(Element::Space),
        };

        // Get filler character
        let filler = match col {
            M::K => Element::Horizontal,
            M::B => Element::Space,
        };

        for _ in 0..max_width + 2 {
            output.push(filler.clone());
        }
    }

    (output, output2)
}

pub fn get_keymap_string(keymap_format: Vec<Element>) -> String {
    keymap_format.iter().map(|x| get_border_name(x)).collect()
}

#[cfg(test)]
mod tests {
    use crate::{ast::LayoutStatement, lexer::TokenType};

    use super::*;

    #[test]
    fn test_key_display() {
        let keymap = KeymapStatement {
            token: TokenType::Ident(4932, "_QWERTY".to_string()),
            layout_statement: LayoutStatement {
                token: TokenType::Layout(4943),
                keys: vec!["KC_ESC".to_string()],
            },
        };

        let layout: Vec<Vec<M>> = vec![vec![M::K]];

        let (display, keymap) = get_keymap_format(&keymap, layout);

        let display = get_keymap_string(display);
        let expected_display =
            "//    ╭────────╮        \n//    │ KC_ESC │        \n//    ╰────────╯";
        assert_eq!(expected_display, display);

        let keymap = get_keymap_string(keymap);
        let expected_keymap = " KC_ESC\n";
        assert_eq!(expected_keymap, keymap);
    }

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
                M::K,
                M::B,
                M::B,
                M::B,
            ],
        ];

        let (display, keymap) = get_keymap_format(&keymap, layout);

        let display = get_keymap_string(display);
        let expected = "//    ╭──────────┬──────────┬──────────┬──────────┬──────────┬──────────╮                                                      ╭──────────┬──────────┬──────────┬──────────┬──────────┬──────────╮          \n//    │ KC_ESC   │ KC_Q     │ KC_W     │ KC_E     │ KC_R     │ KC_T     │                                                      │ KC_Y     │ KC_U     │ KC_I     │ KC_O     │ KC_P     │ KC_BSPC  │          \n//    ├──────────┼──────────┼──────────┼──────────┼──────────┼──────────┤                                                      ├──────────┼──────────┼──────────┼──────────┼──────────┼──────────┤          \n//    │ SFT_TAB  │ KC_A     │ KC_S     │ KC_D     │ KC_F     │ KC_G     │                                                      │ KC_H     │ KC_J     │ KC_K     │ KC_L     │ KC_SCLN  │ KC_QUOTE │          \n//    ├──────────┼──────────┼──────────┼──────────┼──────────┼──────────┼──────────┬──────────╮          ╭──────────┬──────────┼──────────┼──────────┼──────────┼──────────┼──────────┼──────────┤          \n//    │ KC_LCTL  │ KC_Z     │ KC_X     │ KC_C     │ KC_V     │ KC_B     │ KC_CPYP  │ ADJUST   │          │ FKEYS    │          │ KC_N     │ KC_M     │ KC_COMM  │ KC_DOT   │ KC_SLSH  │ KC_RSFT  │          \n//    ╰──────────┴──────────┴──────────┼──────────┼──────────┼──────────┼──────────┼──────────┤          ├──────────┼──────────┼──────────┼──────────┼──────────┼──────────┴──────────┴──────────╯          \n//                                     │ KC_LALT  │ NAV      │ SYM      │ KC_ENT   │ KC_LGUI  │          │ KC_RGUI  │ KC_SPC   │ NAV      │          │          │                                           \n//                                     ╰──────────┴──────────┴──────────┴──────────┴──────────╯          ╰──────────┴──────────┴──────────┴──────────┴──────────╯";
        assert_eq!(expected, display);

        let keymap = get_keymap_string(keymap);
        let expected = " KC_ESC   , KC_Q     , KC_W     , KC_E     , KC_R     , KC_T     ,                                                        KC_Y     , KC_U     , KC_I     , KC_O     , KC_P     , KC_BSPC  ,           \n SFT_TAB  , KC_A     , KC_S     , KC_D     , KC_F     , KC_G     ,                                                        KC_H     , KC_J     , KC_K     , KC_L     , KC_SCLN  , KC_QUOTE ,           \n KC_LCTL  , KC_Z     , KC_X     , KC_C     , KC_V     , KC_B     , KC_CPYP  , ADJUST   ,            FKEYS    , _______  , KC_N     , KC_M     , KC_COMM  , KC_DOT   , KC_SLSH  , KC_RSFT  ,           \n                                  KC_LALT  , NAV      , SYM      , KC_ENT   , KC_LGUI  ,            KC_RGUI  , KC_SPC   , NAV      , _______  , _______";
        assert_eq!(expected, keymap);
    }
}
