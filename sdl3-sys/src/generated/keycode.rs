//! Defines constants which identify keyboard keys and modifiers.

use super::stdinc::*;

use super::scancode::*;

/// The SDL virtual key representation.
///
/// Values of this type are used to represent keyboard keys using the current
/// layout of the keyboard. These values include Unicode values representing
/// the unmodified character that would be generated by pressing the key, or an
/// `SDLK_*` constant for those keys that do not generate characters.
///
/// A special exception is the number keys at the top of the keyboard which map
/// to SDLK_0...SDLK_9 on AZERTY layouts.
///
/// \since This datatype is available since SDL 3.0.0.
pub type SDL_Keycode = Uint32;

pub const SDLK_SCANCODE_MASK: ::core::primitive::u32 = 1073741824_u32;

pub const SDLK_UNKNOWN: ::core::primitive::u32 = 0_u32;

pub const SDLK_RETURN: ::core::primitive::u32 = 13_u32;

pub const SDLK_ESCAPE: ::core::primitive::u32 = 27_u32;

pub const SDLK_BACKSPACE: ::core::primitive::u32 = 8_u32;

pub const SDLK_TAB: ::core::primitive::u32 = 9_u32;

pub const SDLK_SPACE: ::core::primitive::u32 = 32_u32;

pub const SDLK_EXCLAIM: ::core::primitive::u32 = 33_u32;

pub const SDLK_DBLAPOSTROPHE: ::core::primitive::u32 = 34_u32;

pub const SDLK_HASH: ::core::primitive::u32 = 35_u32;

pub const SDLK_DOLLAR: ::core::primitive::u32 = 36_u32;

pub const SDLK_PERCENT: ::core::primitive::u32 = 37_u32;

pub const SDLK_AMPERSAND: ::core::primitive::u32 = 38_u32;

pub const SDLK_APOSTROPHE: ::core::primitive::u32 = 39_u32;

pub const SDLK_LEFTPAREN: ::core::primitive::u32 = 40_u32;

pub const SDLK_RIGHTPAREN: ::core::primitive::u32 = 41_u32;

pub const SDLK_ASTERISK: ::core::primitive::u32 = 42_u32;

pub const SDLK_PLUS: ::core::primitive::u32 = 43_u32;

pub const SDLK_COMMA: ::core::primitive::u32 = 44_u32;

pub const SDLK_MINUS: ::core::primitive::u32 = 45_u32;

pub const SDLK_PERIOD: ::core::primitive::u32 = 46_u32;

pub const SDLK_SLASH: ::core::primitive::u32 = 47_u32;

pub const SDLK_0: ::core::primitive::u32 = 48_u32;

pub const SDLK_1: ::core::primitive::u32 = 49_u32;

pub const SDLK_2: ::core::primitive::u32 = 50_u32;

pub const SDLK_3: ::core::primitive::u32 = 51_u32;

pub const SDLK_4: ::core::primitive::u32 = 52_u32;

pub const SDLK_5: ::core::primitive::u32 = 53_u32;

pub const SDLK_6: ::core::primitive::u32 = 54_u32;

pub const SDLK_7: ::core::primitive::u32 = 55_u32;

pub const SDLK_8: ::core::primitive::u32 = 56_u32;

pub const SDLK_9: ::core::primitive::u32 = 57_u32;

pub const SDLK_COLON: ::core::primitive::u32 = 58_u32;

pub const SDLK_SEMICOLON: ::core::primitive::u32 = 59_u32;

pub const SDLK_LESS: ::core::primitive::u32 = 60_u32;

pub const SDLK_EQUALS: ::core::primitive::u32 = 61_u32;

pub const SDLK_GREATER: ::core::primitive::u32 = 62_u32;

pub const SDLK_QUESTION: ::core::primitive::u32 = 63_u32;

pub const SDLK_AT: ::core::primitive::u32 = 64_u32;

pub const SDLK_LEFTBRACKET: ::core::primitive::u32 = 91_u32;

pub const SDLK_BACKSLASH: ::core::primitive::u32 = 92_u32;

pub const SDLK_RIGHTBRACKET: ::core::primitive::u32 = 93_u32;

pub const SDLK_CARET: ::core::primitive::u32 = 94_u32;

pub const SDLK_UNDERSCORE: ::core::primitive::u32 = 95_u32;

pub const SDLK_GRAVE: ::core::primitive::u32 = 96_u32;

pub const SDLK_A: ::core::primitive::u32 = 97_u32;

pub const SDLK_B: ::core::primitive::u32 = 98_u32;

pub const SDLK_C: ::core::primitive::u32 = 99_u32;

pub const SDLK_D: ::core::primitive::u32 = 100_u32;

pub const SDLK_E: ::core::primitive::u32 = 101_u32;

pub const SDLK_F: ::core::primitive::u32 = 102_u32;

pub const SDLK_G: ::core::primitive::u32 = 103_u32;

pub const SDLK_H: ::core::primitive::u32 = 104_u32;

pub const SDLK_I: ::core::primitive::u32 = 105_u32;

pub const SDLK_J: ::core::primitive::u32 = 106_u32;

pub const SDLK_K: ::core::primitive::u32 = 107_u32;

pub const SDLK_L: ::core::primitive::u32 = 108_u32;

pub const SDLK_M: ::core::primitive::u32 = 109_u32;

pub const SDLK_N: ::core::primitive::u32 = 110_u32;

pub const SDLK_O: ::core::primitive::u32 = 111_u32;

pub const SDLK_P: ::core::primitive::u32 = 112_u32;

pub const SDLK_Q: ::core::primitive::u32 = 113_u32;

pub const SDLK_R: ::core::primitive::u32 = 114_u32;

pub const SDLK_S: ::core::primitive::u32 = 115_u32;

pub const SDLK_T: ::core::primitive::u32 = 116_u32;

pub const SDLK_U: ::core::primitive::u32 = 117_u32;

pub const SDLK_V: ::core::primitive::u32 = 118_u32;

pub const SDLK_W: ::core::primitive::u32 = 119_u32;

pub const SDLK_X: ::core::primitive::u32 = 120_u32;

pub const SDLK_Y: ::core::primitive::u32 = 121_u32;

pub const SDLK_Z: ::core::primitive::u32 = 122_u32;

pub const SDLK_LEFTBRACE: ::core::primitive::u32 = 123_u32;

pub const SDLK_PIPE: ::core::primitive::u32 = 124_u32;

pub const SDLK_RIGHTBRACE: ::core::primitive::u32 = 125_u32;

pub const SDLK_TILDE: ::core::primitive::u32 = 126_u32;

pub const SDLK_DELETE: ::core::primitive::u32 = 127_u32;

pub const SDLK_PLUSMINUS: ::core::primitive::u32 = 177_u32;

pub const SDLK_CAPSLOCK: ::core::primitive::u32 = 1073741881_u32;

pub const SDLK_F1: ::core::primitive::u32 = 1073741882_u32;

pub const SDLK_F2: ::core::primitive::u32 = 1073741883_u32;

pub const SDLK_F3: ::core::primitive::u32 = 1073741884_u32;

pub const SDLK_F4: ::core::primitive::u32 = 1073741885_u32;

pub const SDLK_F5: ::core::primitive::u32 = 1073741886_u32;

pub const SDLK_F6: ::core::primitive::u32 = 1073741887_u32;

pub const SDLK_F7: ::core::primitive::u32 = 1073741888_u32;

pub const SDLK_F8: ::core::primitive::u32 = 1073741889_u32;

pub const SDLK_F9: ::core::primitive::u32 = 1073741890_u32;

pub const SDLK_F10: ::core::primitive::u32 = 1073741891_u32;

pub const SDLK_F11: ::core::primitive::u32 = 1073741892_u32;

pub const SDLK_F12: ::core::primitive::u32 = 1073741893_u32;

pub const SDLK_PRINTSCREEN: ::core::primitive::u32 = 1073741894_u32;

pub const SDLK_SCROLLLOCK: ::core::primitive::u32 = 1073741895_u32;

pub const SDLK_PAUSE: ::core::primitive::u32 = 1073741896_u32;

pub const SDLK_INSERT: ::core::primitive::u32 = 1073741897_u32;

pub const SDLK_HOME: ::core::primitive::u32 = 1073741898_u32;

pub const SDLK_PAGEUP: ::core::primitive::u32 = 1073741899_u32;

pub const SDLK_END: ::core::primitive::u32 = 1073741901_u32;

pub const SDLK_PAGEDOWN: ::core::primitive::u32 = 1073741902_u32;

pub const SDLK_RIGHT: ::core::primitive::u32 = 1073741903_u32;

pub const SDLK_LEFT: ::core::primitive::u32 = 1073741904_u32;

pub const SDLK_DOWN: ::core::primitive::u32 = 1073741905_u32;

pub const SDLK_UP: ::core::primitive::u32 = 1073741906_u32;

pub const SDLK_NUMLOCKCLEAR: ::core::primitive::u32 = 1073741907_u32;

pub const SDLK_KP_DIVIDE: ::core::primitive::u32 = 1073741908_u32;

pub const SDLK_KP_MULTIPLY: ::core::primitive::u32 = 1073741909_u32;

pub const SDLK_KP_MINUS: ::core::primitive::u32 = 1073741910_u32;

pub const SDLK_KP_PLUS: ::core::primitive::u32 = 1073741911_u32;

pub const SDLK_KP_ENTER: ::core::primitive::u32 = 1073741912_u32;

pub const SDLK_KP_1: ::core::primitive::u32 = 1073741913_u32;

pub const SDLK_KP_2: ::core::primitive::u32 = 1073741914_u32;

pub const SDLK_KP_3: ::core::primitive::u32 = 1073741915_u32;

pub const SDLK_KP_4: ::core::primitive::u32 = 1073741916_u32;

pub const SDLK_KP_5: ::core::primitive::u32 = 1073741917_u32;

pub const SDLK_KP_6: ::core::primitive::u32 = 1073741918_u32;

pub const SDLK_KP_7: ::core::primitive::u32 = 1073741919_u32;

pub const SDLK_KP_8: ::core::primitive::u32 = 1073741920_u32;

pub const SDLK_KP_9: ::core::primitive::u32 = 1073741921_u32;

pub const SDLK_KP_0: ::core::primitive::u32 = 1073741922_u32;

pub const SDLK_KP_PERIOD: ::core::primitive::u32 = 1073741923_u32;

pub const SDLK_APPLICATION: ::core::primitive::u32 = 1073741925_u32;

pub const SDLK_POWER: ::core::primitive::u32 = 1073741926_u32;

pub const SDLK_KP_EQUALS: ::core::primitive::u32 = 1073741927_u32;

pub const SDLK_F13: ::core::primitive::u32 = 1073741928_u32;

pub const SDLK_F14: ::core::primitive::u32 = 1073741929_u32;

pub const SDLK_F15: ::core::primitive::u32 = 1073741930_u32;

pub const SDLK_F16: ::core::primitive::u32 = 1073741931_u32;

pub const SDLK_F17: ::core::primitive::u32 = 1073741932_u32;

pub const SDLK_F18: ::core::primitive::u32 = 1073741933_u32;

pub const SDLK_F19: ::core::primitive::u32 = 1073741934_u32;

pub const SDLK_F20: ::core::primitive::u32 = 1073741935_u32;

pub const SDLK_F21: ::core::primitive::u32 = 1073741936_u32;

pub const SDLK_F22: ::core::primitive::u32 = 1073741937_u32;

pub const SDLK_F23: ::core::primitive::u32 = 1073741938_u32;

pub const SDLK_F24: ::core::primitive::u32 = 1073741939_u32;

pub const SDLK_EXECUTE: ::core::primitive::u32 = 1073741940_u32;

pub const SDLK_HELP: ::core::primitive::u32 = 1073741941_u32;

pub const SDLK_MENU: ::core::primitive::u32 = 1073741942_u32;

pub const SDLK_SELECT: ::core::primitive::u32 = 1073741943_u32;

pub const SDLK_STOP: ::core::primitive::u32 = 1073741944_u32;

pub const SDLK_AGAIN: ::core::primitive::u32 = 1073741945_u32;

pub const SDLK_UNDO: ::core::primitive::u32 = 1073741946_u32;

pub const SDLK_CUT: ::core::primitive::u32 = 1073741947_u32;

pub const SDLK_COPY: ::core::primitive::u32 = 1073741948_u32;

pub const SDLK_PASTE: ::core::primitive::u32 = 1073741949_u32;

pub const SDLK_FIND: ::core::primitive::u32 = 1073741950_u32;

pub const SDLK_MUTE: ::core::primitive::u32 = 1073741951_u32;

pub const SDLK_VOLUMEUP: ::core::primitive::u32 = 1073741952_u32;

pub const SDLK_VOLUMEDOWN: ::core::primitive::u32 = 1073741953_u32;

pub const SDLK_KP_COMMA: ::core::primitive::u32 = 1073741957_u32;

pub const SDLK_KP_EQUALSAS400: ::core::primitive::u32 = 1073741958_u32;

pub const SDLK_ALTERASE: ::core::primitive::u32 = 1073741977_u32;

pub const SDLK_SYSREQ: ::core::primitive::u32 = 1073741978_u32;

pub const SDLK_CANCEL: ::core::primitive::u32 = 1073741979_u32;

pub const SDLK_CLEAR: ::core::primitive::u32 = 1073741980_u32;

pub const SDLK_PRIOR: ::core::primitive::u32 = 1073741981_u32;

pub const SDLK_RETURN2: ::core::primitive::u32 = 1073741982_u32;

pub const SDLK_SEPARATOR: ::core::primitive::u32 = 1073741983_u32;

pub const SDLK_OUT: ::core::primitive::u32 = 1073741984_u32;

pub const SDLK_OPER: ::core::primitive::u32 = 1073741985_u32;

pub const SDLK_CLEARAGAIN: ::core::primitive::u32 = 1073741986_u32;

pub const SDLK_CRSEL: ::core::primitive::u32 = 1073741987_u32;

pub const SDLK_EXSEL: ::core::primitive::u32 = 1073741988_u32;

pub const SDLK_KP_00: ::core::primitive::u32 = 1073742000_u32;

pub const SDLK_KP_000: ::core::primitive::u32 = 1073742001_u32;

pub const SDLK_THOUSANDSSEPARATOR: ::core::primitive::u32 = 1073742002_u32;

pub const SDLK_DECIMALSEPARATOR: ::core::primitive::u32 = 1073742003_u32;

pub const SDLK_CURRENCYUNIT: ::core::primitive::u32 = 1073742004_u32;

pub const SDLK_CURRENCYSUBUNIT: ::core::primitive::u32 = 1073742005_u32;

pub const SDLK_KP_LEFTPAREN: ::core::primitive::u32 = 1073742006_u32;

pub const SDLK_KP_RIGHTPAREN: ::core::primitive::u32 = 1073742007_u32;

pub const SDLK_KP_LEFTBRACE: ::core::primitive::u32 = 1073742008_u32;

pub const SDLK_KP_RIGHTBRACE: ::core::primitive::u32 = 1073742009_u32;

pub const SDLK_KP_TAB: ::core::primitive::u32 = 1073742010_u32;

pub const SDLK_KP_BACKSPACE: ::core::primitive::u32 = 1073742011_u32;

pub const SDLK_KP_A: ::core::primitive::u32 = 1073742012_u32;

pub const SDLK_KP_B: ::core::primitive::u32 = 1073742013_u32;

pub const SDLK_KP_C: ::core::primitive::u32 = 1073742014_u32;

pub const SDLK_KP_D: ::core::primitive::u32 = 1073742015_u32;

pub const SDLK_KP_E: ::core::primitive::u32 = 1073742016_u32;

pub const SDLK_KP_F: ::core::primitive::u32 = 1073742017_u32;

pub const SDLK_KP_XOR: ::core::primitive::u32 = 1073742018_u32;

pub const SDLK_KP_POWER: ::core::primitive::u32 = 1073742019_u32;

pub const SDLK_KP_PERCENT: ::core::primitive::u32 = 1073742020_u32;

pub const SDLK_KP_LESS: ::core::primitive::u32 = 1073742021_u32;

pub const SDLK_KP_GREATER: ::core::primitive::u32 = 1073742022_u32;

pub const SDLK_KP_AMPERSAND: ::core::primitive::u32 = 1073742023_u32;

pub const SDLK_KP_DBLAMPERSAND: ::core::primitive::u32 = 1073742024_u32;

pub const SDLK_KP_VERTICALBAR: ::core::primitive::u32 = 1073742025_u32;

pub const SDLK_KP_DBLVERTICALBAR: ::core::primitive::u32 = 1073742026_u32;

pub const SDLK_KP_COLON: ::core::primitive::u32 = 1073742027_u32;

pub const SDLK_KP_HASH: ::core::primitive::u32 = 1073742028_u32;

pub const SDLK_KP_SPACE: ::core::primitive::u32 = 1073742029_u32;

pub const SDLK_KP_AT: ::core::primitive::u32 = 1073742030_u32;

pub const SDLK_KP_EXCLAM: ::core::primitive::u32 = 1073742031_u32;

pub const SDLK_KP_MEMSTORE: ::core::primitive::u32 = 1073742032_u32;

pub const SDLK_KP_MEMRECALL: ::core::primitive::u32 = 1073742033_u32;

pub const SDLK_KP_MEMCLEAR: ::core::primitive::u32 = 1073742034_u32;

pub const SDLK_KP_MEMADD: ::core::primitive::u32 = 1073742035_u32;

pub const SDLK_KP_MEMSUBTRACT: ::core::primitive::u32 = 1073742036_u32;

pub const SDLK_KP_MEMMULTIPLY: ::core::primitive::u32 = 1073742037_u32;

pub const SDLK_KP_MEMDIVIDE: ::core::primitive::u32 = 1073742038_u32;

pub const SDLK_KP_PLUSMINUS: ::core::primitive::u32 = 1073742039_u32;

pub const SDLK_KP_CLEAR: ::core::primitive::u32 = 1073742040_u32;

pub const SDLK_KP_CLEARENTRY: ::core::primitive::u32 = 1073742041_u32;

pub const SDLK_KP_BINARY: ::core::primitive::u32 = 1073742042_u32;

pub const SDLK_KP_OCTAL: ::core::primitive::u32 = 1073742043_u32;

pub const SDLK_KP_DECIMAL: ::core::primitive::u32 = 1073742044_u32;

pub const SDLK_KP_HEXADECIMAL: ::core::primitive::u32 = 1073742045_u32;

pub const SDLK_LCTRL: ::core::primitive::u32 = 1073742048_u32;

pub const SDLK_LSHIFT: ::core::primitive::u32 = 1073742049_u32;

pub const SDLK_LALT: ::core::primitive::u32 = 1073742050_u32;

pub const SDLK_LGUI: ::core::primitive::u32 = 1073742051_u32;

pub const SDLK_RCTRL: ::core::primitive::u32 = 1073742052_u32;

pub const SDLK_RSHIFT: ::core::primitive::u32 = 1073742053_u32;

pub const SDLK_RALT: ::core::primitive::u32 = 1073742054_u32;

pub const SDLK_RGUI: ::core::primitive::u32 = 1073742055_u32;

pub const SDLK_MODE: ::core::primitive::u32 = 1073742081_u32;

pub const SDLK_SLEEP: ::core::primitive::u32 = 1073742082_u32;

pub const SDLK_WAKE: ::core::primitive::u32 = 1073742083_u32;

pub const SDLK_CHANNEL_INCREMENT: ::core::primitive::u32 = 1073742084_u32;

pub const SDLK_CHANNEL_DECREMENT: ::core::primitive::u32 = 1073742085_u32;

pub const SDLK_MEDIA_PLAY: ::core::primitive::u32 = 1073742086_u32;

pub const SDLK_MEDIA_PAUSE: ::core::primitive::u32 = 1073742087_u32;

pub const SDLK_MEDIA_RECORD: ::core::primitive::u32 = 1073742088_u32;

pub const SDLK_MEDIA_FAST_FORWARD: ::core::primitive::u32 = 1073742089_u32;

pub const SDLK_MEDIA_REWIND: ::core::primitive::u32 = 1073742090_u32;

pub const SDLK_MEDIA_NEXT_TRACK: ::core::primitive::u32 = 1073742091_u32;

pub const SDLK_MEDIA_PREVIOUS_TRACK: ::core::primitive::u32 = 1073742092_u32;

pub const SDLK_MEDIA_STOP: ::core::primitive::u32 = 1073742093_u32;

pub const SDLK_MEDIA_EJECT: ::core::primitive::u32 = 1073742094_u32;

pub const SDLK_MEDIA_PLAY_PAUSE: ::core::primitive::u32 = 1073742095_u32;

pub const SDLK_MEDIA_SELECT: ::core::primitive::u32 = 1073742096_u32;

pub const SDLK_AC_NEW: ::core::primitive::u32 = 1073742097_u32;

pub const SDLK_AC_OPEN: ::core::primitive::u32 = 1073742098_u32;

pub const SDLK_AC_CLOSE: ::core::primitive::u32 = 1073742099_u32;

pub const SDLK_AC_EXIT: ::core::primitive::u32 = 1073742100_u32;

pub const SDLK_AC_SAVE: ::core::primitive::u32 = 1073742101_u32;

pub const SDLK_AC_PRINT: ::core::primitive::u32 = 1073742102_u32;

pub const SDLK_AC_PROPERTIES: ::core::primitive::u32 = 1073742103_u32;

pub const SDLK_AC_SEARCH: ::core::primitive::u32 = 1073742104_u32;

pub const SDLK_AC_HOME: ::core::primitive::u32 = 1073742105_u32;

pub const SDLK_AC_BACK: ::core::primitive::u32 = 1073742106_u32;

pub const SDLK_AC_FORWARD: ::core::primitive::u32 = 1073742107_u32;

pub const SDLK_AC_STOP: ::core::primitive::u32 = 1073742108_u32;

pub const SDLK_AC_REFRESH: ::core::primitive::u32 = 1073742109_u32;

pub const SDLK_AC_BOOKMARKS: ::core::primitive::u32 = 1073742110_u32;

pub const SDLK_SOFTLEFT: ::core::primitive::u32 = 1073742111_u32;

pub const SDLK_SOFTRIGHT: ::core::primitive::u32 = 1073742112_u32;

pub const SDLK_CALL: ::core::primitive::u32 = 1073742113_u32;

pub const SDLK_ENDCALL: ::core::primitive::u32 = 1073742114_u32;

/// Valid key modifiers (possibly OR'd together).
///
/// \since This datatype is available since SDL 3.0.0.
pub type SDL_Keymod = Uint16;

/// no modifier is applicable.
pub const SDL_KMOD_NONE: ::core::primitive::u32 = 0_u32;

/// the left Shift key is down.
pub const SDL_KMOD_LSHIFT: ::core::primitive::u32 = 1_u32;

/// the right Shift key is down.
pub const SDL_KMOD_RSHIFT: ::core::primitive::u32 = 2_u32;

/// the left Ctrl (Control) key is down.
pub const SDL_KMOD_LCTRL: ::core::primitive::u32 = 64_u32;

/// the right Ctrl (Control) key is down.
pub const SDL_KMOD_RCTRL: ::core::primitive::u32 = 128_u32;

/// the left Alt key is down.
pub const SDL_KMOD_LALT: ::core::primitive::u32 = 256_u32;

/// the right Alt key is down.
pub const SDL_KMOD_RALT: ::core::primitive::u32 = 512_u32;

/// the left GUI key (often the Windows key) is down.
pub const SDL_KMOD_LGUI: ::core::primitive::u32 = 1024_u32;

/// the right GUI key (often the Windows key) is down.
pub const SDL_KMOD_RGUI: ::core::primitive::u32 = 2048_u32;

/// the Num Lock key (may be located on an extended keypad) is down.
pub const SDL_KMOD_NUM: ::core::primitive::u32 = 4096_u32;

/// the Caps Lock key is down.
pub const SDL_KMOD_CAPS: ::core::primitive::u32 = 8192_u32;

/// the !AltGr key is down.
pub const SDL_KMOD_MODE: ::core::primitive::u32 = 16384_u32;

/// the Scroll Lock key is down.
pub const SDL_KMOD_SCROLL: ::core::primitive::u32 = 32768_u32;

/// Any Ctrl key is down.
pub const SDL_KMOD_CTRL: ::core::primitive::u32 = 192_u32;

/// Any Shift key is down.
pub const SDL_KMOD_SHIFT: ::core::primitive::u32 = 3_u32;

/// Any Alt key is down.
pub const SDL_KMOD_ALT: ::core::primitive::u32 = 768_u32;

/// Any GUI key is down.
pub const SDL_KMOD_GUI: ::core::primitive::u32 = 3072_u32;
