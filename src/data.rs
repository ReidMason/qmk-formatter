pub fn get_keymap() -> String {
    return r##"
/* Copyright 2019 Thomas Baart <thomas@splitkb.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */
#include QMK_KEYBOARD_H
#include "os_detection.h"

enum layers {
    _QWERTY = 0,
    _DVORAK,
    _COLEMAK_DH,
    _NAV,
    _SYM,
    _FUNCTION,
    _ADJUST,
};

#ifdef ENCODER_ENABLE
bool encoder_update_user(uint8_t index, bool clockwise) {
    // This is the left side
    if (index == 0) {
    // The is the right side
      // Page up/Page down
      if (clockwise) {
        tap_code(KC_BRMU);
      } else {
        tap_code(KC_BRMD);
      }
    } else if (index == 1) {
        // Volume control
        if (clockwise) {
          tap_code(KC_VOLU);
        } else {
          tap_code(KC_VOLD);
        }
    }
    return false;
}
#endif

enum my_keycodes {
  HASH = SAFE_RANGE,
  CMD_CTL
};

// Aliases for readability
#define SYM      MO(_SYM)
#define NAV      MO(_NAV)
#define FKEYS    MO(_FUNCTION)
#define ADJUST   MO(_ADJUST)

// #define SFT_TAB  MT(KC_LSFT, KC_TAB)
#define CTL_QUOT MT(MOD_RCTL, KC_QUOTE)
#define CTL_MINS MT(MOD_RCTL, KC_MINUS)
#define ALT_ENT  MT(MOD_LALT, KC_ENT)

#define SFT_TAB LSFT_T(KC_TAB)

#define KC_CAPP LGUI(LSFT(KC_4))        // Capture portion of screen
#define KC_CPYP LGUI(LSFT(LCTL(KC_4)))  // Copy portion of screen

bool process_record_user(uint16_t keycode, keyrecord_t *record) {
  switch (keycode) {
    // Custom hash key for windows and macos
    case HASH:
      if (record->event.pressed) {
        if (detected_host_os() == 3) { 
          tap_code16(LALT(KC_3));
        } else {
          tap_code16(KC_HASH);
        }
      } else {
        // Do something else when release
      }
      return false; // Skip all further processing of this key
    case CMD_CTL:
      if (record->event.pressed) {
        if (detected_host_os() == 3) { 
          tap_code(KC_LGUI);
        } else {
          tap_code(KC_RCTL);
        }
      }
      return false;
    default:
      return true; // Process all other keycodes normally
  }
}

// Note: LAlt/Enter (ALT_ENT) is not the same thing as the keyboard shortcut Alt+Enter.
// The notation `mod/tap` denotes a key that activates the modifier `mod` when held down, and
// produces the key `tap` when tapped (i.e. pressed and released).

// clang-format off
const uint16_t PROGMEM keymaps[][MATRIX_ROWS][MATRIX_COLS] = {
// test

//    ╭──────────┬──────────┬──────────┬──────────┬──────────┬──────────╮                                                      ╭──────────┬──────────┬──────────┬──────────┬──────────┬──────────╮
//    │ KC_ESC   │ KC_Q     │ KC_W     │ KC_E     │ KC_R     │ KC_T     │                                                      │ KC_Y     │ KC_U     │ KC_I     │ KC_O     │ KC_P     │ KC_BSPC  │
//    ├──────────┼──────────┼──────────┼──────────┼──────────┼──────────┤                                                      ├──────────┼──────────┼──────────┼──────────┼──────────┼──────────┤
//    │ SFT_TAB  │ KC_A     │ KC_S     │ KC_D     │ KC_F     │ KC_G     │                                                      │ KC_H     │ KC_J     │ KC_K     │ KC_L     │ KC_SCLN  │ KC_QUOTE │
//    ├──────────┼──────────┼──────────┼──────────┼──────────┼──────────┤                                                      ├──────────┼──────────┼──────────┼──────────┼──────────┼──────────┤
//    │ KC_LCTL  │ KC_Z     │ KC_X     │ KC_C     │ KC_V     │ KC_B     │                                                      │ KC_CPYP  │ ADJUST   │ FKEYS    │          │ KC_N     │ KC_M     │
//    ╰──────────┴──────────┴──────────┼──────────┼──────────┼──────────┼──────────┬──────────╮          ╭──────────┬──────────┼──────────┼──────────┼──────────┴──────────┴──────────┴──────────╯
//                                     │ KC_COMM  │ KC_DOT   │ KC_SLSH  │ KC_RSFT  │ KC_LALT  │          │ NAV      │ SYM      │ KC_ENT   │ KC_LGUI  │
//                                     ╰──────────┴──────────┴──────────┴──────────┴──────────╯          ╰──────────┴──────────┴──────────┴──────────╯
[_QWERTY] = LAYOUT(
  KC_ESC , KC_Q , KC_W , KC_E    , KC_R , KC_T ,                                                 KC_Y , KC_U    , KC_I    , KC_O   , KC_P    , KC_BSPC ,
  SFT_TAB , KC_A , KC_S , KC_D    , KC_F , KC_G ,                                                 KC_H , KC_J    , KC_K    , KC_L   , KC_SCLN , KC_QUOTE,
  KC_LCTL , KC_Z , KC_X , KC_C    , KC_V , KC_B , KC_CPYP , ADJUST  ,         FKEYS   , _______ , KC_N , KC_M    , KC_COMM , KC_DOT , KC_SLSH , KC_RSFT ,
                          KC_LALT , NAV  , SYM  , KC_ENT  , KC_LGUI ,         KC_RGUI , KC_SPC  , NAV  , _______ , _______                              
),
[_NAV] = LAYOUT(
  _______ , _______ , _______ , _______ , _______ , _______ ,                                                 _______ , _______ , _______ , _______ , _______ , KC_DEL ,
  _______ , KC_LGUI , KC_LALT , KC_LCTL , KC_LSFT , _______ ,                                                 KC_LEFT , KC_DOWN , KC_UP   , KC_RGHT , _______ , _______,
  _______ , _______ , _______ , _______ , _______ , _______ , _______ , KC_SCRL ,         _______ , _______ , _______ , _______ , _______ , _______ , _______ , _______,
                                _______ , _______ , _______ , _______ , _______ ,         _______ , _______ , _______ , _______ , _______                              
),

[_SYM] = LAYOUT(
  KC_GRV  , KC_1    , KC_2    , KC_3    , KC_4    , KC_5    ,                                                 KC_6    , KC_7    , KC_8    , KC_9    , KC_0    , KC_EQL ,
  _______ , KC_EXLM , KC_AT   , HASH    , KC_DLR  , KC_MINS ,                                                 KC_UNDS , KC_LPRN , KC_RPRN , KC_LBRC , KC_RBRC , KC_BSLS,
  _______ , KC_BSLS , KC_ASTR , KC_AMPR , KC_PERC , KC_PLUS , _______ , _______ ,         _______ , _______ , _______ , KC_LCBR , KC_RCBR , _______ , _______ , _______,
                                _______ , _______ , _______ , _______ , _______ ,         _______ , _______ , _______ , _______ , _______                              
),

[_FUNCTION] = LAYOUT(
  _______ , KC_F9 , KC_F10 , KC_F11  , KC_F12  , _______ ,                                                 _______ , _______ , _______ , _______ , _______ , _______,
  _______ , KC_F5 , KC_F6  , KC_F7   , KC_F8   , _______ ,                                                 _______ , KC_RSFT , KC_RCTL , KC_LALT , KC_RGUI , _______,
  _______ , KC_F1 , KC_F2  , KC_F3   , KC_F4   , _______ , _______ , _______ ,         _______ , _______ , _______ , _______ , _______ , _______ , _______ , _______,
                             _______ , _______ , _______ , _______ , _______ ,         _______ , _______ , _______ , _______ , _______                              
),

[_ADJUST] = LAYOUT(
  _______ , _______ , _______ , _______ , _______ , _______ ,                                                 _______ , _______ , _______ , _______ , _______  , _______,
  _______ , _______ , _______ , _______ , _______ , _______ ,                                                 RGB_TOG , RGB_SAI , RGB_HUI , RGB_VAI , RGB_MOD  , _______,
  _______ , _______ , _______ , _______ , _______ , _______ , _______ , _______ ,         _______ , _______ , _______ , RGB_SAD , RGB_HUD , RGB_VAD , RGB_RMOD , _______,
                                _______ , _______ , _______ , _______ , _______ ,         _______ , _______ , _______ , _______ , _______                               
)
};

/* The default OLED and rotary encoder code can be found at the bottom of qmk_firmware/keyboards/splitkb/kyria/rev1/rev1.c
 * These default settings can be overriden by your own settings in your keymap.c
 * For your convenience, here's a copy of those settings so that you can uncomment them if you wish to apply your own modifications.
 * DO NOT edit the rev1.c file; instead override the weakly defined default functions by your own.
 */

/* DELETE THIS LINE TO UNCOMMENT (1/2)
#ifdef ENCODER_ENABLE
bool encoder_update_user(uint8_t index, bool clockwise) {

    if (index == 0) {
        // Volume control
        if (clockwise) {
            tap_code(KC_VOLU);
        } else {
            tap_code(KC_VOLD);
        }
    } else if (index == 1) {
        // Page up/Page down
        if (clockwise) {
            tap_code(KC_PGDN);
        } else {
            tap_code(KC_PGUP);
        }
    }
    return false;
}
#endif
DELETE THIS LINE TO UNCOMMENT (2/2) */
"##.to_string();
}
