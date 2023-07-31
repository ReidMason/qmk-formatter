use crate::ast::KeymapStatement;
use Mark::*;

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

impl Element {
    pub fn to_string(&self) -> String {
        match self {
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
}

#[derive(Clone)]
pub enum Mark {
    K,
    B,
}

pub type Layout = Vec<Vec<Mark>>;

pub fn get_keymap_format(keymap: &KeymapStatement, layout: Layout) -> (Vec<Element>, Vec<Element>) {
    // Add an extra blank column on the end to make formatting easier
    let mut layout = layout.clone();
    for row in layout.iter_mut() {
        row.push(Mark::B)
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
        let mut prev: &Mark = &Mark::B;
        if i > 0 {
            prev = match layout[0].get(i - 1) {
                Some(x) => x,
                None => &Mark::B,
            };
        }

        let next = match layout[0].get(i + 1) {
            Some(x) => x,
            None => &Mark::B,
        };

        match (prev, col, next) {
            (K, K, K) => output.push(Element::TopT),
            (K, K, B) => output.push(Element::TopT),
            (K, B, K) => output.push(Element::TopRight),
            (K, B, B) => output.push(Element::TopRight),
            (B, K, K) => output.push(Element::TopLeft),
            (B, K, B) => output.push(Element::TopLeft),
            (B, B, K) => output.push(Element::Space),
            (B, B, B) => output.push(Element::Space),
        };

        // Get filler character
        let filler = match col {
            Mark::K => Element::Horizontal,
            Mark::B => Element::Space,
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
            let mut prev: &Mark = &Mark::B;
            if j > 0 {
                prev = match row.get(j - 1) {
                    Some(x) => x,
                    None => &Mark::B,
                };
            }

            match (prev, col) {
                (K, K) => output.push(Element::Vertical),
                (K, B) => output.push(Element::Vertical),
                (B, K) => output.push(Element::Vertical),
                (B, B) => output.push(Element::Space),
            };

            output.push(Element::Space);
            output2.push(Element::Space);

            let key = match col {
                Mark::K => {
                    let key = &keys[count];
                    output.push(Element::Key(key.to_string()));
                    key
                }
                Mark::B => "",
            };

            for _ in key.len()..max_width {
                output.push(Element::Space);
            }

            let key = match col {
                Mark::K => {
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
                Mark::B => "",
            };

            for _ in key.len()..max_width {
                output2.push(Element::Space);
            }

            output.push(Element::Space);
            output2.push(Element::Space);
            match col {
                Mark::K => {
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
                let mut first: &Mark = &Mark::B;
                if i > 0 && j > 0 {
                    first = match layout.get(i - 1) {
                        Some(x) => match x.get(j - 1) {
                            Some(x) => x,
                            None => &Mark::B,
                        },
                        None => &Mark::B,
                    };
                }

                let mut second: &Mark = &Mark::B;
                if i > 0 {
                    second = match layout.get(i - 1) {
                        Some(x) => match x.get(j) {
                            Some(x) => x,
                            None => &Mark::B,
                        },
                        None => &Mark::B,
                    };
                }

                let mut third: &Mark = &Mark::B;
                if i > 0 {
                    third = match layout.get(i - 1) {
                        Some(x) => match x.get(j + 1) {
                            Some(x) => x,
                            None => &Mark::B,
                        },
                        None => &Mark::B,
                    };
                }

                let mut fourth: &Mark = &Mark::B;
                if j > 0 {
                    fourth = match row.get(j - 1) {
                        Some(x) => x,
                        None => &Mark::B,
                    };
                }

                let sixth = match row.get(j + 1) {
                    Some(x) => x,
                    None => &Mark::B,
                };
                let mut seventh: &Mark = &Mark::B;
                if j > 0 {
                    seventh = match layout.get(i + 1) {
                        Some(x) => match x.get(j - 1) {
                            Some(x) => x,
                            None => &Mark::B,
                        },
                        None => &Mark::B,
                    };
                }

                let eighth = match layout.get(i + 1) {
                    Some(x) => match x.get(j) {
                        Some(x) => x,
                        None => &Mark::B,
                    },
                    None => &Mark::B,
                };

                let ninth = match layout.get(i + 1) {
                    Some(x) => match x.get(j + 1) {
                        Some(x) => x,
                        None => &Mark::B,
                    },
                    None => &Mark::B,
                };

                match (
                    first, second, third, fourth, col, sixth, seventh, eighth, ninth,
                ) {
                    (_, _, _, B, K, _, B, B, _) => output.push(Element::BottomLeft),
                    (_, _, _, K, B, _, B, B, _) => output.push(Element::BottomRight),
                    (_, _, _, K, B, _, K, B, _) => output.push(Element::RightT),
                    (_, _, _, B, K, _, B, K, _) => output.push(Element::LeftT),
                    (_, _, _, K, K, _, B, B, _) => output.push(Element::BottomT),
                    (_, _, _, B, B, _, K, K, _) => output.push(Element::TopT),
                    (_, _, _, B, B, _, K, B, _) => output.push(Element::TopRight),
                    (_, _, _, B, B, _, B, K, _) => output.push(Element::TopLeft),
                    (_, _, _, K, _, _, _, _, _) => output.push(Element::Plus),
                    (_, _, _, _, K, _, _, _, _) => output.push(Element::Plus),
                    _ => output.push(Element::Space),
                };

                // Get filler character
                let filler = match (col, eighth) {
                    (K, K) => Element::Horizontal,
                    (K, B) => Element::Horizontal,
                    (B, K) => Element::Horizontal,
                    (B, B) => Element::Space,
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
        let mut prev: &Mark = &Mark::B;
        if i > 0 {
            prev = match layout[layout.len() - 1].get(i - 1) {
                Some(x) => x,
                None => &Mark::B,
            };
        }

        let next = match layout[layout.len() - 1].get(i + 1) {
            Some(x) => x,
            None => &Mark::B,
        };

        match (prev, col, next) {
            (K, K, K) => output.push(Element::BottomT),
            (K, K, B) => output.push(Element::BottomT),
            (K, B, K) => output.push(Element::BottomRight),
            (K, B, B) => output.push(Element::BottomRight),
            (B, K, K) => output.push(Element::BottomLeft),
            (B, K, B) => output.push(Element::BottomLeft),
            (B, B, K) => output.push(Element::Space),
            (B, B, B) => output.push(Element::Space),
        };

        // Get filler character
        let filler = match col {
            Mark::K => Element::Horizontal,
            Mark::B => Element::Space,
        };

        for _ in 0..max_width + 2 {
            output.push(filler.clone());
        }
    }

    (output, output2)
}

pub fn get_keymap_string(keymap_format: Vec<Element>) -> String {
    keymap_format.iter().map(|x| x.to_string()).collect()
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

        let layout: Vec<Vec<Mark>> = vec![vec![Mark::K]];

        let (display, keymap) = get_keymap_format(&keymap, layout);

        let display = get_keymap_string(display);
        let expected_display =
            "//    ╭────────╮        \n//    │ KC_ESC │        \n//    ╰────────╯";

        for (i, (ex, res)) in expected_display.chars().zip(display.chars()).enumerate() {
            assert_eq!(ex, res, "Char index: {}", i);
        }

        let keymap = get_keymap_string(keymap);
        let expected_keymap = " KC_ESC";
        for (i, (ex, res)) in keymap.chars().zip(expected_keymap.chars()).enumerate() {
            assert_eq!(ex, res, "Char index: {}", i);
        }
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

        let layout: Vec<Vec<Mark>> = vec![
            vec![K, K, K, K, K, K, B, B, B, B, B, K, K, K, K, K, K],
            vec![K, K, K, K, K, K, B, B, B, B, B, K, K, K, K, K, K],
            vec![K, K, K, K, K, K, K, K, B, K, K, K, K, K, K, K, K],
            vec![B, B, B, K, K, K, K, K, B, K, K, K, K, K, B, B, B],
        ];

        let (display, keymap) = get_keymap_format(&keymap, layout);

        let display = get_keymap_string(display);
        let expected = "//    ╭──────────┬──────────┬──────────┬──────────┬──────────┬──────────╮                                                      ╭──────────┬──────────┬──────────┬──────────┬──────────┬──────────╮          \n//    │ KC_ESC   │ KC_Q     │ KC_W     │ KC_E     │ KC_R     │ KC_T     │                                                      │ KC_Y     │ KC_U     │ KC_I     │ KC_O     │ KC_P     │ KC_BSPC  │          \n//    ├──────────┼──────────┼──────────┼──────────┼──────────┼──────────┤                                                      ├──────────┼──────────┼──────────┼──────────┼──────────┼──────────┤          \n//    │ SFT_TAB  │ KC_A     │ KC_S     │ KC_D     │ KC_F     │ KC_G     │                                                      │ KC_H     │ KC_J     │ KC_K     │ KC_L     │ KC_SCLN  │ KC_QUOTE │          \n//    ├──────────┼──────────┼──────────┼──────────┼──────────┼──────────┼──────────┬──────────╮          ╭──────────┬──────────┼──────────┼──────────┼──────────┼──────────┼──────────┼──────────┤          \n//    │ KC_LCTL  │ KC_Z     │ KC_X     │ KC_C     │ KC_V     │ KC_B     │ KC_CPYP  │ ADJUST   │          │ FKEYS    │          │ KC_N     │ KC_M     │ KC_COMM  │ KC_DOT   │ KC_SLSH  │ KC_RSFT  │          \n//    ╰──────────┴──────────┴──────────┼──────────┼──────────┼──────────┼──────────┼──────────┤          ├──────────┼──────────┼──────────┼──────────┼──────────┼──────────┴──────────┴──────────╯          \n//                                     │ KC_LALT  │ NAV      │ SYM      │ KC_ENT   │ KC_LGUI  │          │ KC_RGUI  │ KC_SPC   │ NAV      │          │          │                                           \n//                                     ╰──────────┴──────────┴──────────┴──────────┴──────────╯          ╰──────────┴──────────┴──────────┴──────────┴──────────╯";
        for (ex, res) in expected.chars().zip(display.chars()) {
            assert_eq!(ex, res);
        }

        let keymap = get_keymap_string(keymap);
        let expected = " KC_ESC   , KC_Q     , KC_W     , KC_E     , KC_R     , KC_T     ,                                                        KC_Y     , KC_U     , KC_I     , KC_O     , KC_P     , KC_BSPC  ,           \n SFT_TAB  , KC_A     , KC_S     , KC_D     , KC_F     , KC_G     ,                                                        KC_H     , KC_J     , KC_K     , KC_L     , KC_SCLN  , KC_QUOTE ,           \n KC_LCTL  , KC_Z     , KC_X     , KC_C     , KC_V     , KC_B     , KC_CPYP  , ADJUST   ,            FKEYS    , _______  , KC_N     , KC_M     , KC_COMM  , KC_DOT   , KC_SLSH  , KC_RSFT  ,           \n                                  KC_LALT  , NAV      , SYM      , KC_ENT   , KC_LGUI  ,            KC_RGUI  , KC_SPC   , NAV      , _______  , _______";

        for (ex, res) in expected.chars().zip(keymap.chars()) {
            assert_eq!(ex, res);
        }
    }
}
