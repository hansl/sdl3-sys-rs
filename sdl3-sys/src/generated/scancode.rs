//! Defines keyboard scancodes.

use super::stdinc::*;

/// The SDL keyboard scancode representation.
///
/// An SDL scancode is the physical representation of a key on the keyboard,
/// independent of language and keyboard mapping.
///
/// Values of this type are used to represent keyboard keys, among other places
/// in the `scancode` field of the [`SDL_KeyboardEvent`] structure.
///
/// The values in this enumeration are based on the USB usage page standard:
/// <https://usb.org/sites/default/files/hut1_5.pdf>
///
/// ### Availability
/// This enum is available since SDL 3.0.0.
///
/// ### `sdl3-sys` note
/// This is a `C` enum. Known values:
/// - [`SDL_SCANCODE_A`]
/// - [`SDL_SCANCODE_B`]
/// - [`SDL_SCANCODE_C`]
/// - [`SDL_SCANCODE_D`]
/// - [`SDL_SCANCODE_E`]
/// - [`SDL_SCANCODE_F`]
/// - [`SDL_SCANCODE_G`]
/// - [`SDL_SCANCODE_H`]
/// - [`SDL_SCANCODE_I`]
/// - [`SDL_SCANCODE_J`]
/// - [`SDL_SCANCODE_K`]
/// - [`SDL_SCANCODE_L`]
/// - [`SDL_SCANCODE_M`]
/// - [`SDL_SCANCODE_N`]
/// - [`SDL_SCANCODE_O`]
/// - [`SDL_SCANCODE_P`]
/// - [`SDL_SCANCODE_Q`]
/// - [`SDL_SCANCODE_R`]
/// - [`SDL_SCANCODE_S`]
/// - [`SDL_SCANCODE_T`]
/// - [`SDL_SCANCODE_U`]
/// - [`SDL_SCANCODE_V`]
/// - [`SDL_SCANCODE_W`]
/// - [`SDL_SCANCODE_X`]
/// - [`SDL_SCANCODE_Y`]
/// - [`SDL_SCANCODE_Z`]
/// - [`SDL_SCANCODE_1`]
/// - [`SDL_SCANCODE_2`]
/// - [`SDL_SCANCODE_3`]
/// - [`SDL_SCANCODE_4`]
/// - [`SDL_SCANCODE_5`]
/// - [`SDL_SCANCODE_6`]
/// - [`SDL_SCANCODE_7`]
/// - [`SDL_SCANCODE_8`]
/// - [`SDL_SCANCODE_9`]
/// - [`SDL_SCANCODE_0`]
/// - [`SDL_SCANCODE_RETURN`]
/// - [`SDL_SCANCODE_ESCAPE`]
/// - [`SDL_SCANCODE_BACKSPACE`]
/// - [`SDL_SCANCODE_TAB`]
/// - [`SDL_SCANCODE_SPACE`]
/// - [`SDL_SCANCODE_MINUS`]
/// - [`SDL_SCANCODE_EQUALS`]
/// - [`SDL_SCANCODE_LEFTBRACKET`]
/// - [`SDL_SCANCODE_RIGHTBRACKET`]
/// - [`SDL_SCANCODE_BACKSLASH`]
/// - [`SDL_SCANCODE_NONUSHASH`]
/// - [`SDL_SCANCODE_SEMICOLON`]
/// - [`SDL_SCANCODE_APOSTROPHE`]
/// - [`SDL_SCANCODE_GRAVE`]
/// - [`SDL_SCANCODE_COMMA`]
/// - [`SDL_SCANCODE_PERIOD`]
/// - [`SDL_SCANCODE_SLASH`]
/// - [`SDL_SCANCODE_CAPSLOCK`]
/// - [`SDL_SCANCODE_F1`]
/// - [`SDL_SCANCODE_F2`]
/// - [`SDL_SCANCODE_F3`]
/// - [`SDL_SCANCODE_F4`]
/// - [`SDL_SCANCODE_F5`]
/// - [`SDL_SCANCODE_F6`]
/// - [`SDL_SCANCODE_F7`]
/// - [`SDL_SCANCODE_F8`]
/// - [`SDL_SCANCODE_F9`]
/// - [`SDL_SCANCODE_F10`]
/// - [`SDL_SCANCODE_F11`]
/// - [`SDL_SCANCODE_F12`]
/// - [`SDL_SCANCODE_PRINTSCREEN`]
/// - [`SDL_SCANCODE_SCROLLLOCK`]
/// - [`SDL_SCANCODE_PAUSE`]
/// - [`SDL_SCANCODE_INSERT`]
/// - [`SDL_SCANCODE_HOME`]
/// - [`SDL_SCANCODE_PAGEUP`]
/// - [`SDL_SCANCODE_DELETE`]
/// - [`SDL_SCANCODE_END`]
/// - [`SDL_SCANCODE_PAGEDOWN`]
/// - [`SDL_SCANCODE_RIGHT`]
/// - [`SDL_SCANCODE_LEFT`]
/// - [`SDL_SCANCODE_DOWN`]
/// - [`SDL_SCANCODE_UP`]
/// - [`SDL_SCANCODE_NUMLOCKCLEAR`]
/// - [`SDL_SCANCODE_KP_DIVIDE`]
/// - [`SDL_SCANCODE_KP_MULTIPLY`]
/// - [`SDL_SCANCODE_KP_MINUS`]
/// - [`SDL_SCANCODE_KP_PLUS`]
/// - [`SDL_SCANCODE_KP_ENTER`]
/// - [`SDL_SCANCODE_KP_1`]
/// - [`SDL_SCANCODE_KP_2`]
/// - [`SDL_SCANCODE_KP_3`]
/// - [`SDL_SCANCODE_KP_4`]
/// - [`SDL_SCANCODE_KP_5`]
/// - [`SDL_SCANCODE_KP_6`]
/// - [`SDL_SCANCODE_KP_7`]
/// - [`SDL_SCANCODE_KP_8`]
/// - [`SDL_SCANCODE_KP_9`]
/// - [`SDL_SCANCODE_KP_0`]
/// - [`SDL_SCANCODE_KP_PERIOD`]
/// - [`SDL_SCANCODE_NONUSBACKSLASH`]
/// - [`SDL_SCANCODE_APPLICATION`]
/// - [`SDL_SCANCODE_POWER`]
/// - [`SDL_SCANCODE_KP_EQUALS`]
/// - [`SDL_SCANCODE_F13`]
/// - [`SDL_SCANCODE_F14`]
/// - [`SDL_SCANCODE_F15`]
/// - [`SDL_SCANCODE_F16`]
/// - [`SDL_SCANCODE_F17`]
/// - [`SDL_SCANCODE_F18`]
/// - [`SDL_SCANCODE_F19`]
/// - [`SDL_SCANCODE_F20`]
/// - [`SDL_SCANCODE_F21`]
/// - [`SDL_SCANCODE_F22`]
/// - [`SDL_SCANCODE_F23`]
/// - [`SDL_SCANCODE_F24`]
/// - [`SDL_SCANCODE_EXECUTE`]
/// - [`SDL_SCANCODE_HELP`]
/// - [`SDL_SCANCODE_MENU`]
/// - [`SDL_SCANCODE_SELECT`]
/// - [`SDL_SCANCODE_STOP`]
/// - [`SDL_SCANCODE_AGAIN`]
/// - [`SDL_SCANCODE_UNDO`]
/// - [`SDL_SCANCODE_CUT`]
/// - [`SDL_SCANCODE_COPY`]
/// - [`SDL_SCANCODE_PASTE`]
/// - [`SDL_SCANCODE_FIND`]
/// - [`SDL_SCANCODE_MUTE`]
/// - [`SDL_SCANCODE_VOLUMEUP`]
/// - [`SDL_SCANCODE_VOLUMEDOWN`]
/// - [`SDL_SCANCODE_KP_COMMA`]
/// - [`SDL_SCANCODE_KP_EQUALSAS400`]
/// - [`SDL_SCANCODE_INTERNATIONAL1`]
/// - [`SDL_SCANCODE_INTERNATIONAL2`]
/// - [`SDL_SCANCODE_INTERNATIONAL3`]
/// - [`SDL_SCANCODE_INTERNATIONAL4`]
/// - [`SDL_SCANCODE_INTERNATIONAL5`]
/// - [`SDL_SCANCODE_INTERNATIONAL6`]
/// - [`SDL_SCANCODE_INTERNATIONAL7`]
/// - [`SDL_SCANCODE_INTERNATIONAL8`]
/// - [`SDL_SCANCODE_INTERNATIONAL9`]
/// - [`SDL_SCANCODE_LANG1`]
/// - [`SDL_SCANCODE_LANG2`]
/// - [`SDL_SCANCODE_LANG3`]
/// - [`SDL_SCANCODE_LANG4`]
/// - [`SDL_SCANCODE_LANG5`]
/// - [`SDL_SCANCODE_LANG6`]
/// - [`SDL_SCANCODE_LANG7`]
/// - [`SDL_SCANCODE_LANG8`]
/// - [`SDL_SCANCODE_LANG9`]
/// - [`SDL_SCANCODE_ALTERASE`]
/// - [`SDL_SCANCODE_SYSREQ`]
/// - [`SDL_SCANCODE_CANCEL`]
/// - [`SDL_SCANCODE_CLEAR`]
/// - [`SDL_SCANCODE_PRIOR`]
/// - [`SDL_SCANCODE_RETURN2`]
/// - [`SDL_SCANCODE_SEPARATOR`]
/// - [`SDL_SCANCODE_OUT`]
/// - [`SDL_SCANCODE_OPER`]
/// - [`SDL_SCANCODE_CLEARAGAIN`]
/// - [`SDL_SCANCODE_CRSEL`]
/// - [`SDL_SCANCODE_EXSEL`]
/// - [`SDL_SCANCODE_KP_00`]
/// - [`SDL_SCANCODE_KP_000`]
/// - [`SDL_SCANCODE_THOUSANDSSEPARATOR`]
/// - [`SDL_SCANCODE_DECIMALSEPARATOR`]
/// - [`SDL_SCANCODE_CURRENCYUNIT`]
/// - [`SDL_SCANCODE_CURRENCYSUBUNIT`]
/// - [`SDL_SCANCODE_KP_LEFTPAREN`]
/// - [`SDL_SCANCODE_KP_RIGHTPAREN`]
/// - [`SDL_SCANCODE_KP_LEFTBRACE`]
/// - [`SDL_SCANCODE_KP_RIGHTBRACE`]
/// - [`SDL_SCANCODE_KP_TAB`]
/// - [`SDL_SCANCODE_KP_BACKSPACE`]
/// - [`SDL_SCANCODE_KP_A`]
/// - [`SDL_SCANCODE_KP_B`]
/// - [`SDL_SCANCODE_KP_C`]
/// - [`SDL_SCANCODE_KP_D`]
/// - [`SDL_SCANCODE_KP_E`]
/// - [`SDL_SCANCODE_KP_F`]
/// - [`SDL_SCANCODE_KP_XOR`]
/// - [`SDL_SCANCODE_KP_POWER`]
/// - [`SDL_SCANCODE_KP_PERCENT`]
/// - [`SDL_SCANCODE_KP_LESS`]
/// - [`SDL_SCANCODE_KP_GREATER`]
/// - [`SDL_SCANCODE_KP_AMPERSAND`]
/// - [`SDL_SCANCODE_KP_DBLAMPERSAND`]
/// - [`SDL_SCANCODE_KP_VERTICALBAR`]
/// - [`SDL_SCANCODE_KP_DBLVERTICALBAR`]
/// - [`SDL_SCANCODE_KP_COLON`]
/// - [`SDL_SCANCODE_KP_HASH`]
/// - [`SDL_SCANCODE_KP_SPACE`]
/// - [`SDL_SCANCODE_KP_AT`]
/// - [`SDL_SCANCODE_KP_EXCLAM`]
/// - [`SDL_SCANCODE_KP_MEMSTORE`]
/// - [`SDL_SCANCODE_KP_MEMRECALL`]
/// - [`SDL_SCANCODE_KP_MEMCLEAR`]
/// - [`SDL_SCANCODE_KP_MEMADD`]
/// - [`SDL_SCANCODE_KP_MEMSUBTRACT`]
/// - [`SDL_SCANCODE_KP_MEMMULTIPLY`]
/// - [`SDL_SCANCODE_KP_MEMDIVIDE`]
/// - [`SDL_SCANCODE_KP_PLUSMINUS`]
/// - [`SDL_SCANCODE_KP_CLEAR`]
/// - [`SDL_SCANCODE_KP_CLEARENTRY`]
/// - [`SDL_SCANCODE_KP_BINARY`]
/// - [`SDL_SCANCODE_KP_OCTAL`]
/// - [`SDL_SCANCODE_KP_DECIMAL`]
/// - [`SDL_SCANCODE_KP_HEXADECIMAL`]
/// - [`SDL_SCANCODE_LCTRL`]
/// - [`SDL_SCANCODE_LSHIFT`]
/// - [`SDL_SCANCODE_LALT`]
/// - [`SDL_SCANCODE_LGUI`]
/// - [`SDL_SCANCODE_RCTRL`]
/// - [`SDL_SCANCODE_RSHIFT`]
/// - [`SDL_SCANCODE_RALT`]
/// - [`SDL_SCANCODE_RGUI`]
/// - [`SDL_SCANCODE_MODE`]
/// - [`SDL_SCANCODE_SLEEP`]
/// - [`SDL_SCANCODE_WAKE`]
/// - [`SDL_SCANCODE_CHANNEL_INCREMENT`]
/// - [`SDL_SCANCODE_CHANNEL_DECREMENT`]
/// - [`SDL_SCANCODE_MEDIA_PLAY`]
/// - [`SDL_SCANCODE_MEDIA_PAUSE`]
/// - [`SDL_SCANCODE_MEDIA_RECORD`]
/// - [`SDL_SCANCODE_MEDIA_FAST_FORWARD`]
/// - [`SDL_SCANCODE_MEDIA_REWIND`]
/// - [`SDL_SCANCODE_MEDIA_NEXT_TRACK`]
/// - [`SDL_SCANCODE_MEDIA_PREVIOUS_TRACK`]
/// - [`SDL_SCANCODE_MEDIA_STOP`]
/// - [`SDL_SCANCODE_MEDIA_EJECT`]
/// - [`SDL_SCANCODE_MEDIA_PLAY_PAUSE`]
/// - [`SDL_SCANCODE_MEDIA_SELECT`]
/// - [`SDL_SCANCODE_AC_NEW`]
/// - [`SDL_SCANCODE_AC_OPEN`]
/// - [`SDL_SCANCODE_AC_CLOSE`]
/// - [`SDL_SCANCODE_AC_EXIT`]
/// - [`SDL_SCANCODE_AC_SAVE`]
/// - [`SDL_SCANCODE_AC_PRINT`]
/// - [`SDL_SCANCODE_AC_PROPERTIES`]
/// - [`SDL_SCANCODE_AC_SEARCH`]
/// - [`SDL_SCANCODE_AC_HOME`]
/// - [`SDL_SCANCODE_AC_BACK`]
/// - [`SDL_SCANCODE_AC_FORWARD`]
/// - [`SDL_SCANCODE_AC_STOP`]
/// - [`SDL_SCANCODE_AC_REFRESH`]
/// - [`SDL_SCANCODE_AC_BOOKMARKS`]
/// - [`SDL_SCANCODE_SOFTLEFT`]
/// - [`SDL_SCANCODE_SOFTRIGHT`]
/// - [`SDL_SCANCODE_CALL`]
/// - [`SDL_SCANCODE_ENDCALL`]
/// - [`SDL_SCANCODE_RESERVED`]
/// - [`SDL_SCANCODE_COUNT`]

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_Scancode(pub ::core::ffi::c_int);
impl From<SDL_Scancode> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_Scancode) -> Self {
        value.0
    }
}
impl SDL_Scancode {
    pub const UNKNOWN: Self = Self(0);
    pub const A: Self = Self(4);
    pub const B: Self = Self(5);
    pub const C: Self = Self(6);
    pub const D: Self = Self(7);
    pub const E: Self = Self(8);
    pub const F: Self = Self(9);
    pub const G: Self = Self(10);
    pub const H: Self = Self(11);
    pub const I: Self = Self(12);
    pub const J: Self = Self(13);
    pub const K: Self = Self(14);
    pub const L: Self = Self(15);
    pub const M: Self = Self(16);
    pub const N: Self = Self(17);
    pub const O: Self = Self(18);
    pub const P: Self = Self(19);
    pub const Q: Self = Self(20);
    pub const R: Self = Self(21);
    pub const S: Self = Self(22);
    pub const T: Self = Self(23);
    pub const U: Self = Self(24);
    pub const V: Self = Self(25);
    pub const W: Self = Self(26);
    pub const X: Self = Self(27);
    pub const Y: Self = Self(28);
    pub const Z: Self = Self(29);
    pub const _1: Self = Self(30);
    pub const _2: Self = Self(31);
    pub const _3: Self = Self(32);
    pub const _4: Self = Self(33);
    pub const _5: Self = Self(34);
    pub const _6: Self = Self(35);
    pub const _7: Self = Self(36);
    pub const _8: Self = Self(37);
    pub const _9: Self = Self(38);
    pub const _0: Self = Self(39);
    pub const RETURN: Self = Self(40);
    pub const ESCAPE: Self = Self(41);
    pub const BACKSPACE: Self = Self(42);
    pub const TAB: Self = Self(43);
    pub const SPACE: Self = Self(44);
    pub const MINUS: Self = Self(45);
    pub const EQUALS: Self = Self(46);
    pub const LEFTBRACKET: Self = Self(47);
    pub const RIGHTBRACKET: Self = Self(48);
    /// Located at the lower left of the return
    /// key on ISO keyboards and at the right end
    /// of the QWERTY row on ANSI keyboards.
    /// Produces REVERSE SOLIDUS (backslash) and
    /// VERTICAL LINE in a US layout, REVERSE
    /// SOLIDUS and VERTICAL LINE in a UK Mac
    /// layout, NUMBER SIGN and TILDE in a UK
    /// Windows layout, DOLLAR SIGN and POUND SIGN
    /// in a Swiss German layout, NUMBER SIGN and
    /// APOSTROPHE in a German layout, GRAVE
    /// ACCENT and POUND SIGN in a French Mac
    /// layout, and ASTERISK and MICRO SIGN in a
    /// French Windows layout.
    pub const BACKSLASH: Self = Self(49);
    /// ISO USB keyboards actually use this code
    /// instead of 49 for the same key, but all
    /// OSes I've seen treat the two codes
    /// identically. So, as an implementor, unless
    /// your keyboard generates both of those
    /// codes and your OS treats them differently,
    /// you should generate [`SDL_SCANCODE_BACKSLASH`]
    /// instead of this code. As a user, you
    /// should not rely on this code because SDL
    /// will never generate it with most (all?)
    /// keyboards.
    pub const NONUSHASH: Self = Self(50);
    pub const SEMICOLON: Self = Self(51);
    pub const APOSTROPHE: Self = Self(52);
    /// Located in the top left corner (on both ANSI
    /// and ISO keyboards). Produces GRAVE ACCENT and
    /// TILDE in a US Windows layout and in US and UK
    /// Mac layouts on ANSI keyboards, GRAVE ACCENT
    /// and NOT SIGN in a UK Windows layout, SECTION
    /// SIGN and PLUS-MINUS SIGN in US and UK Mac
    /// layouts on ISO keyboards, SECTION SIGN and
    /// DEGREE SIGN in a Swiss German layout (Mac:
    /// only on ISO keyboards), CIRCUMFLEX ACCENT and
    /// DEGREE SIGN in a German layout (Mac: only on
    /// ISO keyboards), SUPERSCRIPT TWO and TILDE in a
    /// French Windows layout, COMMERCIAL AT and
    /// NUMBER SIGN in a French Mac layout on ISO
    /// keyboards, and LESS-THAN SIGN and GREATER-THAN
    /// SIGN in a Swiss German, German, or French Mac
    /// layout on ANSI keyboards.
    pub const GRAVE: Self = Self(53);
    pub const COMMA: Self = Self(54);
    pub const PERIOD: Self = Self(55);
    pub const SLASH: Self = Self(56);
    pub const CAPSLOCK: Self = Self(57);
    pub const F1: Self = Self(58);
    pub const F2: Self = Self(59);
    pub const F3: Self = Self(60);
    pub const F4: Self = Self(61);
    pub const F5: Self = Self(62);
    pub const F6: Self = Self(63);
    pub const F7: Self = Self(64);
    pub const F8: Self = Self(65);
    pub const F9: Self = Self(66);
    pub const F10: Self = Self(67);
    pub const F11: Self = Self(68);
    pub const F12: Self = Self(69);
    pub const PRINTSCREEN: Self = Self(70);
    pub const SCROLLLOCK: Self = Self(71);
    pub const PAUSE: Self = Self(72);
    /// insert on PC, help on some Mac keyboards (but
    /// does send code 73, not 117)
    pub const INSERT: Self = Self(73);
    pub const HOME: Self = Self(74);
    pub const PAGEUP: Self = Self(75);
    pub const DELETE: Self = Self(76);
    pub const END: Self = Self(77);
    pub const PAGEDOWN: Self = Self(78);
    pub const RIGHT: Self = Self(79);
    pub const LEFT: Self = Self(80);
    pub const DOWN: Self = Self(81);
    pub const UP: Self = Self(82);
    /// num lock on PC, clear on Mac keyboards
    pub const NUMLOCKCLEAR: Self = Self(83);
    pub const KP_DIVIDE: Self = Self(84);
    pub const KP_MULTIPLY: Self = Self(85);
    pub const KP_MINUS: Self = Self(86);
    pub const KP_PLUS: Self = Self(87);
    pub const KP_ENTER: Self = Self(88);
    pub const KP_1: Self = Self(89);
    pub const KP_2: Self = Self(90);
    pub const KP_3: Self = Self(91);
    pub const KP_4: Self = Self(92);
    pub const KP_5: Self = Self(93);
    pub const KP_6: Self = Self(94);
    pub const KP_7: Self = Self(95);
    pub const KP_8: Self = Self(96);
    pub const KP_9: Self = Self(97);
    pub const KP_0: Self = Self(98);
    pub const KP_PERIOD: Self = Self(99);
    /// This is the additional key that ISO
    /// keyboards have over ANSI ones,
    /// located between left shift and Y.
    /// Produces GRAVE ACCENT and TILDE in a
    /// US or UK Mac layout, REVERSE SOLIDUS
    /// (backslash) and VERTICAL LINE in a
    /// US or UK Windows layout, and
    /// LESS-THAN SIGN and GREATER-THAN SIGN
    /// in a Swiss German, German, or French
    /// layout.
    pub const NONUSBACKSLASH: Self = Self(100);
    /// windows contextual menu, compose
    pub const APPLICATION: Self = Self(101);
    /// The USB document says this is a status flag,
    /// not a physical key - but some Mac keyboards
    /// do have a power key.
    pub const POWER: Self = Self(102);
    pub const KP_EQUALS: Self = Self(103);
    pub const F13: Self = Self(104);
    pub const F14: Self = Self(105);
    pub const F15: Self = Self(106);
    pub const F16: Self = Self(107);
    pub const F17: Self = Self(108);
    pub const F18: Self = Self(109);
    pub const F19: Self = Self(110);
    pub const F20: Self = Self(111);
    pub const F21: Self = Self(112);
    pub const F22: Self = Self(113);
    pub const F23: Self = Self(114);
    pub const F24: Self = Self(115);
    pub const EXECUTE: Self = Self(116);
    /// AL Integrated Help Center
    pub const HELP: Self = Self(117);
    /// Menu (show menu)
    pub const MENU: Self = Self(118);
    pub const SELECT: Self = Self(119);
    /// AC Stop
    pub const STOP: Self = Self(120);
    /// AC Redo/Repeat
    pub const AGAIN: Self = Self(121);
    /// AC Undo
    pub const UNDO: Self = Self(122);
    /// AC Cut
    pub const CUT: Self = Self(123);
    /// AC Copy
    pub const COPY: Self = Self(124);
    /// AC Paste
    pub const PASTE: Self = Self(125);
    /// AC Find
    pub const FIND: Self = Self(126);
    pub const MUTE: Self = Self(127);
    pub const VOLUMEUP: Self = Self(128);
    pub const VOLUMEDOWN: Self = Self(129);
    pub const KP_COMMA: Self = Self(133);
    pub const KP_EQUALSAS400: Self = Self(134);
    /// used on Asian keyboards, see
    /// footnotes in USB doc
    pub const INTERNATIONAL1: Self = Self(135);
    pub const INTERNATIONAL2: Self = Self(136);
    /// Yen
    pub const INTERNATIONAL3: Self = Self(137);
    pub const INTERNATIONAL4: Self = Self(138);
    pub const INTERNATIONAL5: Self = Self(139);
    pub const INTERNATIONAL6: Self = Self(140);
    pub const INTERNATIONAL7: Self = Self(141);
    pub const INTERNATIONAL8: Self = Self(142);
    pub const INTERNATIONAL9: Self = Self(143);
    /// Hangul/English toggle
    pub const LANG1: Self = Self(144);
    /// Hanja conversion
    pub const LANG2: Self = Self(145);
    /// Katakana
    pub const LANG3: Self = Self(146);
    /// Hiragana
    pub const LANG4: Self = Self(147);
    /// Zenkaku/Hankaku
    pub const LANG5: Self = Self(148);
    /// reserved
    pub const LANG6: Self = Self(149);
    /// reserved
    pub const LANG7: Self = Self(150);
    /// reserved
    pub const LANG8: Self = Self(151);
    /// reserved
    pub const LANG9: Self = Self(152);
    /// Erase-Eaze
    pub const ALTERASE: Self = Self(153);
    pub const SYSREQ: Self = Self(154);
    /// AC Cancel
    pub const CANCEL: Self = Self(155);
    pub const CLEAR: Self = Self(156);
    pub const PRIOR: Self = Self(157);
    pub const RETURN2: Self = Self(158);
    pub const SEPARATOR: Self = Self(159);
    pub const OUT: Self = Self(160);
    pub const OPER: Self = Self(161);
    pub const CLEARAGAIN: Self = Self(162);
    pub const CRSEL: Self = Self(163);
    pub const EXSEL: Self = Self(164);
    pub const KP_00: Self = Self(176);
    pub const KP_000: Self = Self(177);
    pub const THOUSANDSSEPARATOR: Self = Self(178);
    pub const DECIMALSEPARATOR: Self = Self(179);
    pub const CURRENCYUNIT: Self = Self(180);
    pub const CURRENCYSUBUNIT: Self = Self(181);
    pub const KP_LEFTPAREN: Self = Self(182);
    pub const KP_RIGHTPAREN: Self = Self(183);
    pub const KP_LEFTBRACE: Self = Self(184);
    pub const KP_RIGHTBRACE: Self = Self(185);
    pub const KP_TAB: Self = Self(186);
    pub const KP_BACKSPACE: Self = Self(187);
    pub const KP_A: Self = Self(188);
    pub const KP_B: Self = Self(189);
    pub const KP_C: Self = Self(190);
    pub const KP_D: Self = Self(191);
    pub const KP_E: Self = Self(192);
    pub const KP_F: Self = Self(193);
    pub const KP_XOR: Self = Self(194);
    pub const KP_POWER: Self = Self(195);
    pub const KP_PERCENT: Self = Self(196);
    pub const KP_LESS: Self = Self(197);
    pub const KP_GREATER: Self = Self(198);
    pub const KP_AMPERSAND: Self = Self(199);
    pub const KP_DBLAMPERSAND: Self = Self(200);
    pub const KP_VERTICALBAR: Self = Self(201);
    pub const KP_DBLVERTICALBAR: Self = Self(202);
    pub const KP_COLON: Self = Self(203);
    pub const KP_HASH: Self = Self(204);
    pub const KP_SPACE: Self = Self(205);
    pub const KP_AT: Self = Self(206);
    pub const KP_EXCLAM: Self = Self(207);
    pub const KP_MEMSTORE: Self = Self(208);
    pub const KP_MEMRECALL: Self = Self(209);
    pub const KP_MEMCLEAR: Self = Self(210);
    pub const KP_MEMADD: Self = Self(211);
    pub const KP_MEMSUBTRACT: Self = Self(212);
    pub const KP_MEMMULTIPLY: Self = Self(213);
    pub const KP_MEMDIVIDE: Self = Self(214);
    pub const KP_PLUSMINUS: Self = Self(215);
    pub const KP_CLEAR: Self = Self(216);
    pub const KP_CLEARENTRY: Self = Self(217);
    pub const KP_BINARY: Self = Self(218);
    pub const KP_OCTAL: Self = Self(219);
    pub const KP_DECIMAL: Self = Self(220);
    pub const KP_HEXADECIMAL: Self = Self(221);
    pub const LCTRL: Self = Self(224);
    pub const LSHIFT: Self = Self(225);
    /// alt, option
    pub const LALT: Self = Self(226);
    /// windows, command (apple), meta
    pub const LGUI: Self = Self(227);
    pub const RCTRL: Self = Self(228);
    pub const RSHIFT: Self = Self(229);
    /// alt gr, option
    pub const RALT: Self = Self(230);
    /// windows, command (apple), meta
    pub const RGUI: Self = Self(231);
    /// I'm not sure if this is really not covered
    /// by any of the above, but since there's a
    /// special [`SDL_KMOD_MODE`] for it I'm adding it here
    pub const MODE: Self = Self(257);
    /// Sleep
    pub const SLEEP: Self = Self(258);
    /// Wake
    pub const WAKE: Self = Self(259);
    /// Channel Increment
    pub const CHANNEL_INCREMENT: Self = Self(260);
    /// Channel Decrement
    pub const CHANNEL_DECREMENT: Self = Self(261);
    /// Play
    pub const MEDIA_PLAY: Self = Self(262);
    /// Pause
    pub const MEDIA_PAUSE: Self = Self(263);
    /// Record
    pub const MEDIA_RECORD: Self = Self(264);
    /// Fast Forward
    pub const MEDIA_FAST_FORWARD: Self = Self(265);
    /// Rewind
    pub const MEDIA_REWIND: Self = Self(266);
    /// Next Track
    pub const MEDIA_NEXT_TRACK: Self = Self(267);
    /// Previous Track
    pub const MEDIA_PREVIOUS_TRACK: Self = Self(268);
    /// Stop
    pub const MEDIA_STOP: Self = Self(269);
    /// Eject
    pub const MEDIA_EJECT: Self = Self(270);
    /// Play / Pause
    pub const MEDIA_PLAY_PAUSE: Self = Self(271);
    pub const MEDIA_SELECT: Self = Self(272);
    /// AC New
    pub const AC_NEW: Self = Self(273);
    /// AC Open
    pub const AC_OPEN: Self = Self(274);
    /// AC Close
    pub const AC_CLOSE: Self = Self(275);
    /// AC Exit
    pub const AC_EXIT: Self = Self(276);
    /// AC Save
    pub const AC_SAVE: Self = Self(277);
    /// AC Print
    pub const AC_PRINT: Self = Self(278);
    /// AC Properties
    pub const AC_PROPERTIES: Self = Self(279);
    /// AC Search
    pub const AC_SEARCH: Self = Self(280);
    /// AC Home
    pub const AC_HOME: Self = Self(281);
    /// AC Back
    pub const AC_BACK: Self = Self(282);
    /// AC Forward
    pub const AC_FORWARD: Self = Self(283);
    /// AC Stop
    pub const AC_STOP: Self = Self(284);
    /// AC Refresh
    pub const AC_REFRESH: Self = Self(285);
    /// AC Bookmarks
    pub const AC_BOOKMARKS: Self = Self(286);
    /// Usually situated below the display on phones and
    /// used as a multi-function feature key for selecting
    /// a software defined function shown on the bottom left
    /// of the display.
    pub const SOFTLEFT: Self = Self(287);
    /// Usually situated below the display on phones and
    /// used as a multi-function feature key for selecting
    /// a software defined function shown on the bottom right
    /// of the display.
    pub const SOFTRIGHT: Self = Self(288);
    /// Used for accepting phone calls.
    pub const CALL: Self = Self(289);
    /// Used for rejecting phone calls.
    pub const ENDCALL: Self = Self(290);
    /// 400-500 reserved for dynamic keycodes
    pub const RESERVED: Self = Self(400);
    /// not a key, just marks the number of scancodes for array bounds
    pub const COUNT: Self = Self(512);
}
pub const SDL_SCANCODE_UNKNOWN: SDL_Scancode = SDL_Scancode::UNKNOWN;
pub const SDL_SCANCODE_A: SDL_Scancode = SDL_Scancode::A;
pub const SDL_SCANCODE_B: SDL_Scancode = SDL_Scancode::B;
pub const SDL_SCANCODE_C: SDL_Scancode = SDL_Scancode::C;
pub const SDL_SCANCODE_D: SDL_Scancode = SDL_Scancode::D;
pub const SDL_SCANCODE_E: SDL_Scancode = SDL_Scancode::E;
pub const SDL_SCANCODE_F: SDL_Scancode = SDL_Scancode::F;
pub const SDL_SCANCODE_G: SDL_Scancode = SDL_Scancode::G;
pub const SDL_SCANCODE_H: SDL_Scancode = SDL_Scancode::H;
pub const SDL_SCANCODE_I: SDL_Scancode = SDL_Scancode::I;
pub const SDL_SCANCODE_J: SDL_Scancode = SDL_Scancode::J;
pub const SDL_SCANCODE_K: SDL_Scancode = SDL_Scancode::K;
pub const SDL_SCANCODE_L: SDL_Scancode = SDL_Scancode::L;
pub const SDL_SCANCODE_M: SDL_Scancode = SDL_Scancode::M;
pub const SDL_SCANCODE_N: SDL_Scancode = SDL_Scancode::N;
pub const SDL_SCANCODE_O: SDL_Scancode = SDL_Scancode::O;
pub const SDL_SCANCODE_P: SDL_Scancode = SDL_Scancode::P;
pub const SDL_SCANCODE_Q: SDL_Scancode = SDL_Scancode::Q;
pub const SDL_SCANCODE_R: SDL_Scancode = SDL_Scancode::R;
pub const SDL_SCANCODE_S: SDL_Scancode = SDL_Scancode::S;
pub const SDL_SCANCODE_T: SDL_Scancode = SDL_Scancode::T;
pub const SDL_SCANCODE_U: SDL_Scancode = SDL_Scancode::U;
pub const SDL_SCANCODE_V: SDL_Scancode = SDL_Scancode::V;
pub const SDL_SCANCODE_W: SDL_Scancode = SDL_Scancode::W;
pub const SDL_SCANCODE_X: SDL_Scancode = SDL_Scancode::X;
pub const SDL_SCANCODE_Y: SDL_Scancode = SDL_Scancode::Y;
pub const SDL_SCANCODE_Z: SDL_Scancode = SDL_Scancode::Z;
pub const SDL_SCANCODE_1: SDL_Scancode = SDL_Scancode::_1;
pub const SDL_SCANCODE_2: SDL_Scancode = SDL_Scancode::_2;
pub const SDL_SCANCODE_3: SDL_Scancode = SDL_Scancode::_3;
pub const SDL_SCANCODE_4: SDL_Scancode = SDL_Scancode::_4;
pub const SDL_SCANCODE_5: SDL_Scancode = SDL_Scancode::_5;
pub const SDL_SCANCODE_6: SDL_Scancode = SDL_Scancode::_6;
pub const SDL_SCANCODE_7: SDL_Scancode = SDL_Scancode::_7;
pub const SDL_SCANCODE_8: SDL_Scancode = SDL_Scancode::_8;
pub const SDL_SCANCODE_9: SDL_Scancode = SDL_Scancode::_9;
pub const SDL_SCANCODE_0: SDL_Scancode = SDL_Scancode::_0;
pub const SDL_SCANCODE_RETURN: SDL_Scancode = SDL_Scancode::RETURN;
pub const SDL_SCANCODE_ESCAPE: SDL_Scancode = SDL_Scancode::ESCAPE;
pub const SDL_SCANCODE_BACKSPACE: SDL_Scancode = SDL_Scancode::BACKSPACE;
pub const SDL_SCANCODE_TAB: SDL_Scancode = SDL_Scancode::TAB;
pub const SDL_SCANCODE_SPACE: SDL_Scancode = SDL_Scancode::SPACE;
pub const SDL_SCANCODE_MINUS: SDL_Scancode = SDL_Scancode::MINUS;
pub const SDL_SCANCODE_EQUALS: SDL_Scancode = SDL_Scancode::EQUALS;
pub const SDL_SCANCODE_LEFTBRACKET: SDL_Scancode = SDL_Scancode::LEFTBRACKET;
pub const SDL_SCANCODE_RIGHTBRACKET: SDL_Scancode = SDL_Scancode::RIGHTBRACKET;
/// Located at the lower left of the return
/// key on ISO keyboards and at the right end
/// of the QWERTY row on ANSI keyboards.
/// Produces REVERSE SOLIDUS (backslash) and
/// VERTICAL LINE in a US layout, REVERSE
/// SOLIDUS and VERTICAL LINE in a UK Mac
/// layout, NUMBER SIGN and TILDE in a UK
/// Windows layout, DOLLAR SIGN and POUND SIGN
/// in a Swiss German layout, NUMBER SIGN and
/// APOSTROPHE in a German layout, GRAVE
/// ACCENT and POUND SIGN in a French Mac
/// layout, and ASTERISK and MICRO SIGN in a
/// French Windows layout.
pub const SDL_SCANCODE_BACKSLASH: SDL_Scancode = SDL_Scancode::BACKSLASH;
/// ISO USB keyboards actually use this code
/// instead of 49 for the same key, but all
/// OSes I've seen treat the two codes
/// identically. So, as an implementor, unless
/// your keyboard generates both of those
/// codes and your OS treats them differently,
/// you should generate [`SDL_SCANCODE_BACKSLASH`]
/// instead of this code. As a user, you
/// should not rely on this code because SDL
/// will never generate it with most (all?)
/// keyboards.
pub const SDL_SCANCODE_NONUSHASH: SDL_Scancode = SDL_Scancode::NONUSHASH;
pub const SDL_SCANCODE_SEMICOLON: SDL_Scancode = SDL_Scancode::SEMICOLON;
pub const SDL_SCANCODE_APOSTROPHE: SDL_Scancode = SDL_Scancode::APOSTROPHE;
/// Located in the top left corner (on both ANSI
/// and ISO keyboards). Produces GRAVE ACCENT and
/// TILDE in a US Windows layout and in US and UK
/// Mac layouts on ANSI keyboards, GRAVE ACCENT
/// and NOT SIGN in a UK Windows layout, SECTION
/// SIGN and PLUS-MINUS SIGN in US and UK Mac
/// layouts on ISO keyboards, SECTION SIGN and
/// DEGREE SIGN in a Swiss German layout (Mac:
/// only on ISO keyboards), CIRCUMFLEX ACCENT and
/// DEGREE SIGN in a German layout (Mac: only on
/// ISO keyboards), SUPERSCRIPT TWO and TILDE in a
/// French Windows layout, COMMERCIAL AT and
/// NUMBER SIGN in a French Mac layout on ISO
/// keyboards, and LESS-THAN SIGN and GREATER-THAN
/// SIGN in a Swiss German, German, or French Mac
/// layout on ANSI keyboards.
pub const SDL_SCANCODE_GRAVE: SDL_Scancode = SDL_Scancode::GRAVE;
pub const SDL_SCANCODE_COMMA: SDL_Scancode = SDL_Scancode::COMMA;
pub const SDL_SCANCODE_PERIOD: SDL_Scancode = SDL_Scancode::PERIOD;
pub const SDL_SCANCODE_SLASH: SDL_Scancode = SDL_Scancode::SLASH;
pub const SDL_SCANCODE_CAPSLOCK: SDL_Scancode = SDL_Scancode::CAPSLOCK;
pub const SDL_SCANCODE_F1: SDL_Scancode = SDL_Scancode::F1;
pub const SDL_SCANCODE_F2: SDL_Scancode = SDL_Scancode::F2;
pub const SDL_SCANCODE_F3: SDL_Scancode = SDL_Scancode::F3;
pub const SDL_SCANCODE_F4: SDL_Scancode = SDL_Scancode::F4;
pub const SDL_SCANCODE_F5: SDL_Scancode = SDL_Scancode::F5;
pub const SDL_SCANCODE_F6: SDL_Scancode = SDL_Scancode::F6;
pub const SDL_SCANCODE_F7: SDL_Scancode = SDL_Scancode::F7;
pub const SDL_SCANCODE_F8: SDL_Scancode = SDL_Scancode::F8;
pub const SDL_SCANCODE_F9: SDL_Scancode = SDL_Scancode::F9;
pub const SDL_SCANCODE_F10: SDL_Scancode = SDL_Scancode::F10;
pub const SDL_SCANCODE_F11: SDL_Scancode = SDL_Scancode::F11;
pub const SDL_SCANCODE_F12: SDL_Scancode = SDL_Scancode::F12;
pub const SDL_SCANCODE_PRINTSCREEN: SDL_Scancode = SDL_Scancode::PRINTSCREEN;
pub const SDL_SCANCODE_SCROLLLOCK: SDL_Scancode = SDL_Scancode::SCROLLLOCK;
pub const SDL_SCANCODE_PAUSE: SDL_Scancode = SDL_Scancode::PAUSE;
/// insert on PC, help on some Mac keyboards (but
/// does send code 73, not 117)
pub const SDL_SCANCODE_INSERT: SDL_Scancode = SDL_Scancode::INSERT;
pub const SDL_SCANCODE_HOME: SDL_Scancode = SDL_Scancode::HOME;
pub const SDL_SCANCODE_PAGEUP: SDL_Scancode = SDL_Scancode::PAGEUP;
pub const SDL_SCANCODE_DELETE: SDL_Scancode = SDL_Scancode::DELETE;
pub const SDL_SCANCODE_END: SDL_Scancode = SDL_Scancode::END;
pub const SDL_SCANCODE_PAGEDOWN: SDL_Scancode = SDL_Scancode::PAGEDOWN;
pub const SDL_SCANCODE_RIGHT: SDL_Scancode = SDL_Scancode::RIGHT;
pub const SDL_SCANCODE_LEFT: SDL_Scancode = SDL_Scancode::LEFT;
pub const SDL_SCANCODE_DOWN: SDL_Scancode = SDL_Scancode::DOWN;
pub const SDL_SCANCODE_UP: SDL_Scancode = SDL_Scancode::UP;
/// num lock on PC, clear on Mac keyboards
pub const SDL_SCANCODE_NUMLOCKCLEAR: SDL_Scancode = SDL_Scancode::NUMLOCKCLEAR;
pub const SDL_SCANCODE_KP_DIVIDE: SDL_Scancode = SDL_Scancode::KP_DIVIDE;
pub const SDL_SCANCODE_KP_MULTIPLY: SDL_Scancode = SDL_Scancode::KP_MULTIPLY;
pub const SDL_SCANCODE_KP_MINUS: SDL_Scancode = SDL_Scancode::KP_MINUS;
pub const SDL_SCANCODE_KP_PLUS: SDL_Scancode = SDL_Scancode::KP_PLUS;
pub const SDL_SCANCODE_KP_ENTER: SDL_Scancode = SDL_Scancode::KP_ENTER;
pub const SDL_SCANCODE_KP_1: SDL_Scancode = SDL_Scancode::KP_1;
pub const SDL_SCANCODE_KP_2: SDL_Scancode = SDL_Scancode::KP_2;
pub const SDL_SCANCODE_KP_3: SDL_Scancode = SDL_Scancode::KP_3;
pub const SDL_SCANCODE_KP_4: SDL_Scancode = SDL_Scancode::KP_4;
pub const SDL_SCANCODE_KP_5: SDL_Scancode = SDL_Scancode::KP_5;
pub const SDL_SCANCODE_KP_6: SDL_Scancode = SDL_Scancode::KP_6;
pub const SDL_SCANCODE_KP_7: SDL_Scancode = SDL_Scancode::KP_7;
pub const SDL_SCANCODE_KP_8: SDL_Scancode = SDL_Scancode::KP_8;
pub const SDL_SCANCODE_KP_9: SDL_Scancode = SDL_Scancode::KP_9;
pub const SDL_SCANCODE_KP_0: SDL_Scancode = SDL_Scancode::KP_0;
pub const SDL_SCANCODE_KP_PERIOD: SDL_Scancode = SDL_Scancode::KP_PERIOD;
/// This is the additional key that ISO
/// keyboards have over ANSI ones,
/// located between left shift and Y.
/// Produces GRAVE ACCENT and TILDE in a
/// US or UK Mac layout, REVERSE SOLIDUS
/// (backslash) and VERTICAL LINE in a
/// US or UK Windows layout, and
/// LESS-THAN SIGN and GREATER-THAN SIGN
/// in a Swiss German, German, or French
/// layout.
pub const SDL_SCANCODE_NONUSBACKSLASH: SDL_Scancode = SDL_Scancode::NONUSBACKSLASH;
/// windows contextual menu, compose
pub const SDL_SCANCODE_APPLICATION: SDL_Scancode = SDL_Scancode::APPLICATION;
/// The USB document says this is a status flag,
/// not a physical key - but some Mac keyboards
/// do have a power key.
pub const SDL_SCANCODE_POWER: SDL_Scancode = SDL_Scancode::POWER;
pub const SDL_SCANCODE_KP_EQUALS: SDL_Scancode = SDL_Scancode::KP_EQUALS;
pub const SDL_SCANCODE_F13: SDL_Scancode = SDL_Scancode::F13;
pub const SDL_SCANCODE_F14: SDL_Scancode = SDL_Scancode::F14;
pub const SDL_SCANCODE_F15: SDL_Scancode = SDL_Scancode::F15;
pub const SDL_SCANCODE_F16: SDL_Scancode = SDL_Scancode::F16;
pub const SDL_SCANCODE_F17: SDL_Scancode = SDL_Scancode::F17;
pub const SDL_SCANCODE_F18: SDL_Scancode = SDL_Scancode::F18;
pub const SDL_SCANCODE_F19: SDL_Scancode = SDL_Scancode::F19;
pub const SDL_SCANCODE_F20: SDL_Scancode = SDL_Scancode::F20;
pub const SDL_SCANCODE_F21: SDL_Scancode = SDL_Scancode::F21;
pub const SDL_SCANCODE_F22: SDL_Scancode = SDL_Scancode::F22;
pub const SDL_SCANCODE_F23: SDL_Scancode = SDL_Scancode::F23;
pub const SDL_SCANCODE_F24: SDL_Scancode = SDL_Scancode::F24;
pub const SDL_SCANCODE_EXECUTE: SDL_Scancode = SDL_Scancode::EXECUTE;
/// AL Integrated Help Center
pub const SDL_SCANCODE_HELP: SDL_Scancode = SDL_Scancode::HELP;
/// Menu (show menu)
pub const SDL_SCANCODE_MENU: SDL_Scancode = SDL_Scancode::MENU;
pub const SDL_SCANCODE_SELECT: SDL_Scancode = SDL_Scancode::SELECT;
/// AC Stop
pub const SDL_SCANCODE_STOP: SDL_Scancode = SDL_Scancode::STOP;
/// AC Redo/Repeat
pub const SDL_SCANCODE_AGAIN: SDL_Scancode = SDL_Scancode::AGAIN;
/// AC Undo
pub const SDL_SCANCODE_UNDO: SDL_Scancode = SDL_Scancode::UNDO;
/// AC Cut
pub const SDL_SCANCODE_CUT: SDL_Scancode = SDL_Scancode::CUT;
/// AC Copy
pub const SDL_SCANCODE_COPY: SDL_Scancode = SDL_Scancode::COPY;
/// AC Paste
pub const SDL_SCANCODE_PASTE: SDL_Scancode = SDL_Scancode::PASTE;
/// AC Find
pub const SDL_SCANCODE_FIND: SDL_Scancode = SDL_Scancode::FIND;
pub const SDL_SCANCODE_MUTE: SDL_Scancode = SDL_Scancode::MUTE;
pub const SDL_SCANCODE_VOLUMEUP: SDL_Scancode = SDL_Scancode::VOLUMEUP;
pub const SDL_SCANCODE_VOLUMEDOWN: SDL_Scancode = SDL_Scancode::VOLUMEDOWN;
pub const SDL_SCANCODE_KP_COMMA: SDL_Scancode = SDL_Scancode::KP_COMMA;
pub const SDL_SCANCODE_KP_EQUALSAS400: SDL_Scancode = SDL_Scancode::KP_EQUALSAS400;
/// used on Asian keyboards, see
/// footnotes in USB doc
pub const SDL_SCANCODE_INTERNATIONAL1: SDL_Scancode = SDL_Scancode::INTERNATIONAL1;
pub const SDL_SCANCODE_INTERNATIONAL2: SDL_Scancode = SDL_Scancode::INTERNATIONAL2;
/// Yen
pub const SDL_SCANCODE_INTERNATIONAL3: SDL_Scancode = SDL_Scancode::INTERNATIONAL3;
pub const SDL_SCANCODE_INTERNATIONAL4: SDL_Scancode = SDL_Scancode::INTERNATIONAL4;
pub const SDL_SCANCODE_INTERNATIONAL5: SDL_Scancode = SDL_Scancode::INTERNATIONAL5;
pub const SDL_SCANCODE_INTERNATIONAL6: SDL_Scancode = SDL_Scancode::INTERNATIONAL6;
pub const SDL_SCANCODE_INTERNATIONAL7: SDL_Scancode = SDL_Scancode::INTERNATIONAL7;
pub const SDL_SCANCODE_INTERNATIONAL8: SDL_Scancode = SDL_Scancode::INTERNATIONAL8;
pub const SDL_SCANCODE_INTERNATIONAL9: SDL_Scancode = SDL_Scancode::INTERNATIONAL9;
/// Hangul/English toggle
pub const SDL_SCANCODE_LANG1: SDL_Scancode = SDL_Scancode::LANG1;
/// Hanja conversion
pub const SDL_SCANCODE_LANG2: SDL_Scancode = SDL_Scancode::LANG2;
/// Katakana
pub const SDL_SCANCODE_LANG3: SDL_Scancode = SDL_Scancode::LANG3;
/// Hiragana
pub const SDL_SCANCODE_LANG4: SDL_Scancode = SDL_Scancode::LANG4;
/// Zenkaku/Hankaku
pub const SDL_SCANCODE_LANG5: SDL_Scancode = SDL_Scancode::LANG5;
/// reserved
pub const SDL_SCANCODE_LANG6: SDL_Scancode = SDL_Scancode::LANG6;
/// reserved
pub const SDL_SCANCODE_LANG7: SDL_Scancode = SDL_Scancode::LANG7;
/// reserved
pub const SDL_SCANCODE_LANG8: SDL_Scancode = SDL_Scancode::LANG8;
/// reserved
pub const SDL_SCANCODE_LANG9: SDL_Scancode = SDL_Scancode::LANG9;
/// Erase-Eaze
pub const SDL_SCANCODE_ALTERASE: SDL_Scancode = SDL_Scancode::ALTERASE;
pub const SDL_SCANCODE_SYSREQ: SDL_Scancode = SDL_Scancode::SYSREQ;
/// AC Cancel
pub const SDL_SCANCODE_CANCEL: SDL_Scancode = SDL_Scancode::CANCEL;
pub const SDL_SCANCODE_CLEAR: SDL_Scancode = SDL_Scancode::CLEAR;
pub const SDL_SCANCODE_PRIOR: SDL_Scancode = SDL_Scancode::PRIOR;
pub const SDL_SCANCODE_RETURN2: SDL_Scancode = SDL_Scancode::RETURN2;
pub const SDL_SCANCODE_SEPARATOR: SDL_Scancode = SDL_Scancode::SEPARATOR;
pub const SDL_SCANCODE_OUT: SDL_Scancode = SDL_Scancode::OUT;
pub const SDL_SCANCODE_OPER: SDL_Scancode = SDL_Scancode::OPER;
pub const SDL_SCANCODE_CLEARAGAIN: SDL_Scancode = SDL_Scancode::CLEARAGAIN;
pub const SDL_SCANCODE_CRSEL: SDL_Scancode = SDL_Scancode::CRSEL;
pub const SDL_SCANCODE_EXSEL: SDL_Scancode = SDL_Scancode::EXSEL;
pub const SDL_SCANCODE_KP_00: SDL_Scancode = SDL_Scancode::KP_00;
pub const SDL_SCANCODE_KP_000: SDL_Scancode = SDL_Scancode::KP_000;
pub const SDL_SCANCODE_THOUSANDSSEPARATOR: SDL_Scancode = SDL_Scancode::THOUSANDSSEPARATOR;
pub const SDL_SCANCODE_DECIMALSEPARATOR: SDL_Scancode = SDL_Scancode::DECIMALSEPARATOR;
pub const SDL_SCANCODE_CURRENCYUNIT: SDL_Scancode = SDL_Scancode::CURRENCYUNIT;
pub const SDL_SCANCODE_CURRENCYSUBUNIT: SDL_Scancode = SDL_Scancode::CURRENCYSUBUNIT;
pub const SDL_SCANCODE_KP_LEFTPAREN: SDL_Scancode = SDL_Scancode::KP_LEFTPAREN;
pub const SDL_SCANCODE_KP_RIGHTPAREN: SDL_Scancode = SDL_Scancode::KP_RIGHTPAREN;
pub const SDL_SCANCODE_KP_LEFTBRACE: SDL_Scancode = SDL_Scancode::KP_LEFTBRACE;
pub const SDL_SCANCODE_KP_RIGHTBRACE: SDL_Scancode = SDL_Scancode::KP_RIGHTBRACE;
pub const SDL_SCANCODE_KP_TAB: SDL_Scancode = SDL_Scancode::KP_TAB;
pub const SDL_SCANCODE_KP_BACKSPACE: SDL_Scancode = SDL_Scancode::KP_BACKSPACE;
pub const SDL_SCANCODE_KP_A: SDL_Scancode = SDL_Scancode::KP_A;
pub const SDL_SCANCODE_KP_B: SDL_Scancode = SDL_Scancode::KP_B;
pub const SDL_SCANCODE_KP_C: SDL_Scancode = SDL_Scancode::KP_C;
pub const SDL_SCANCODE_KP_D: SDL_Scancode = SDL_Scancode::KP_D;
pub const SDL_SCANCODE_KP_E: SDL_Scancode = SDL_Scancode::KP_E;
pub const SDL_SCANCODE_KP_F: SDL_Scancode = SDL_Scancode::KP_F;
pub const SDL_SCANCODE_KP_XOR: SDL_Scancode = SDL_Scancode::KP_XOR;
pub const SDL_SCANCODE_KP_POWER: SDL_Scancode = SDL_Scancode::KP_POWER;
pub const SDL_SCANCODE_KP_PERCENT: SDL_Scancode = SDL_Scancode::KP_PERCENT;
pub const SDL_SCANCODE_KP_LESS: SDL_Scancode = SDL_Scancode::KP_LESS;
pub const SDL_SCANCODE_KP_GREATER: SDL_Scancode = SDL_Scancode::KP_GREATER;
pub const SDL_SCANCODE_KP_AMPERSAND: SDL_Scancode = SDL_Scancode::KP_AMPERSAND;
pub const SDL_SCANCODE_KP_DBLAMPERSAND: SDL_Scancode = SDL_Scancode::KP_DBLAMPERSAND;
pub const SDL_SCANCODE_KP_VERTICALBAR: SDL_Scancode = SDL_Scancode::KP_VERTICALBAR;
pub const SDL_SCANCODE_KP_DBLVERTICALBAR: SDL_Scancode = SDL_Scancode::KP_DBLVERTICALBAR;
pub const SDL_SCANCODE_KP_COLON: SDL_Scancode = SDL_Scancode::KP_COLON;
pub const SDL_SCANCODE_KP_HASH: SDL_Scancode = SDL_Scancode::KP_HASH;
pub const SDL_SCANCODE_KP_SPACE: SDL_Scancode = SDL_Scancode::KP_SPACE;
pub const SDL_SCANCODE_KP_AT: SDL_Scancode = SDL_Scancode::KP_AT;
pub const SDL_SCANCODE_KP_EXCLAM: SDL_Scancode = SDL_Scancode::KP_EXCLAM;
pub const SDL_SCANCODE_KP_MEMSTORE: SDL_Scancode = SDL_Scancode::KP_MEMSTORE;
pub const SDL_SCANCODE_KP_MEMRECALL: SDL_Scancode = SDL_Scancode::KP_MEMRECALL;
pub const SDL_SCANCODE_KP_MEMCLEAR: SDL_Scancode = SDL_Scancode::KP_MEMCLEAR;
pub const SDL_SCANCODE_KP_MEMADD: SDL_Scancode = SDL_Scancode::KP_MEMADD;
pub const SDL_SCANCODE_KP_MEMSUBTRACT: SDL_Scancode = SDL_Scancode::KP_MEMSUBTRACT;
pub const SDL_SCANCODE_KP_MEMMULTIPLY: SDL_Scancode = SDL_Scancode::KP_MEMMULTIPLY;
pub const SDL_SCANCODE_KP_MEMDIVIDE: SDL_Scancode = SDL_Scancode::KP_MEMDIVIDE;
pub const SDL_SCANCODE_KP_PLUSMINUS: SDL_Scancode = SDL_Scancode::KP_PLUSMINUS;
pub const SDL_SCANCODE_KP_CLEAR: SDL_Scancode = SDL_Scancode::KP_CLEAR;
pub const SDL_SCANCODE_KP_CLEARENTRY: SDL_Scancode = SDL_Scancode::KP_CLEARENTRY;
pub const SDL_SCANCODE_KP_BINARY: SDL_Scancode = SDL_Scancode::KP_BINARY;
pub const SDL_SCANCODE_KP_OCTAL: SDL_Scancode = SDL_Scancode::KP_OCTAL;
pub const SDL_SCANCODE_KP_DECIMAL: SDL_Scancode = SDL_Scancode::KP_DECIMAL;
pub const SDL_SCANCODE_KP_HEXADECIMAL: SDL_Scancode = SDL_Scancode::KP_HEXADECIMAL;
pub const SDL_SCANCODE_LCTRL: SDL_Scancode = SDL_Scancode::LCTRL;
pub const SDL_SCANCODE_LSHIFT: SDL_Scancode = SDL_Scancode::LSHIFT;
/// alt, option
pub const SDL_SCANCODE_LALT: SDL_Scancode = SDL_Scancode::LALT;
/// windows, command (apple), meta
pub const SDL_SCANCODE_LGUI: SDL_Scancode = SDL_Scancode::LGUI;
pub const SDL_SCANCODE_RCTRL: SDL_Scancode = SDL_Scancode::RCTRL;
pub const SDL_SCANCODE_RSHIFT: SDL_Scancode = SDL_Scancode::RSHIFT;
/// alt gr, option
pub const SDL_SCANCODE_RALT: SDL_Scancode = SDL_Scancode::RALT;
/// windows, command (apple), meta
pub const SDL_SCANCODE_RGUI: SDL_Scancode = SDL_Scancode::RGUI;
/// I'm not sure if this is really not covered
/// by any of the above, but since there's a
/// special [`SDL_KMOD_MODE`] for it I'm adding it here
pub const SDL_SCANCODE_MODE: SDL_Scancode = SDL_Scancode::MODE;
/// Sleep
pub const SDL_SCANCODE_SLEEP: SDL_Scancode = SDL_Scancode::SLEEP;
/// Wake
pub const SDL_SCANCODE_WAKE: SDL_Scancode = SDL_Scancode::WAKE;
/// Channel Increment
pub const SDL_SCANCODE_CHANNEL_INCREMENT: SDL_Scancode = SDL_Scancode::CHANNEL_INCREMENT;
/// Channel Decrement
pub const SDL_SCANCODE_CHANNEL_DECREMENT: SDL_Scancode = SDL_Scancode::CHANNEL_DECREMENT;
/// Play
pub const SDL_SCANCODE_MEDIA_PLAY: SDL_Scancode = SDL_Scancode::MEDIA_PLAY;
/// Pause
pub const SDL_SCANCODE_MEDIA_PAUSE: SDL_Scancode = SDL_Scancode::MEDIA_PAUSE;
/// Record
pub const SDL_SCANCODE_MEDIA_RECORD: SDL_Scancode = SDL_Scancode::MEDIA_RECORD;
/// Fast Forward
pub const SDL_SCANCODE_MEDIA_FAST_FORWARD: SDL_Scancode = SDL_Scancode::MEDIA_FAST_FORWARD;
/// Rewind
pub const SDL_SCANCODE_MEDIA_REWIND: SDL_Scancode = SDL_Scancode::MEDIA_REWIND;
/// Next Track
pub const SDL_SCANCODE_MEDIA_NEXT_TRACK: SDL_Scancode = SDL_Scancode::MEDIA_NEXT_TRACK;
/// Previous Track
pub const SDL_SCANCODE_MEDIA_PREVIOUS_TRACK: SDL_Scancode = SDL_Scancode::MEDIA_PREVIOUS_TRACK;
/// Stop
pub const SDL_SCANCODE_MEDIA_STOP: SDL_Scancode = SDL_Scancode::MEDIA_STOP;
/// Eject
pub const SDL_SCANCODE_MEDIA_EJECT: SDL_Scancode = SDL_Scancode::MEDIA_EJECT;
/// Play / Pause
pub const SDL_SCANCODE_MEDIA_PLAY_PAUSE: SDL_Scancode = SDL_Scancode::MEDIA_PLAY_PAUSE;
pub const SDL_SCANCODE_MEDIA_SELECT: SDL_Scancode = SDL_Scancode::MEDIA_SELECT;
/// AC New
pub const SDL_SCANCODE_AC_NEW: SDL_Scancode = SDL_Scancode::AC_NEW;
/// AC Open
pub const SDL_SCANCODE_AC_OPEN: SDL_Scancode = SDL_Scancode::AC_OPEN;
/// AC Close
pub const SDL_SCANCODE_AC_CLOSE: SDL_Scancode = SDL_Scancode::AC_CLOSE;
/// AC Exit
pub const SDL_SCANCODE_AC_EXIT: SDL_Scancode = SDL_Scancode::AC_EXIT;
/// AC Save
pub const SDL_SCANCODE_AC_SAVE: SDL_Scancode = SDL_Scancode::AC_SAVE;
/// AC Print
pub const SDL_SCANCODE_AC_PRINT: SDL_Scancode = SDL_Scancode::AC_PRINT;
/// AC Properties
pub const SDL_SCANCODE_AC_PROPERTIES: SDL_Scancode = SDL_Scancode::AC_PROPERTIES;
/// AC Search
pub const SDL_SCANCODE_AC_SEARCH: SDL_Scancode = SDL_Scancode::AC_SEARCH;
/// AC Home
pub const SDL_SCANCODE_AC_HOME: SDL_Scancode = SDL_Scancode::AC_HOME;
/// AC Back
pub const SDL_SCANCODE_AC_BACK: SDL_Scancode = SDL_Scancode::AC_BACK;
/// AC Forward
pub const SDL_SCANCODE_AC_FORWARD: SDL_Scancode = SDL_Scancode::AC_FORWARD;
/// AC Stop
pub const SDL_SCANCODE_AC_STOP: SDL_Scancode = SDL_Scancode::AC_STOP;
/// AC Refresh
pub const SDL_SCANCODE_AC_REFRESH: SDL_Scancode = SDL_Scancode::AC_REFRESH;
/// AC Bookmarks
pub const SDL_SCANCODE_AC_BOOKMARKS: SDL_Scancode = SDL_Scancode::AC_BOOKMARKS;
/// Usually situated below the display on phones and
/// used as a multi-function feature key for selecting
/// a software defined function shown on the bottom left
/// of the display.
pub const SDL_SCANCODE_SOFTLEFT: SDL_Scancode = SDL_Scancode::SOFTLEFT;
/// Usually situated below the display on phones and
/// used as a multi-function feature key for selecting
/// a software defined function shown on the bottom right
/// of the display.
pub const SDL_SCANCODE_SOFTRIGHT: SDL_Scancode = SDL_Scancode::SOFTRIGHT;
/// Used for accepting phone calls.
pub const SDL_SCANCODE_CALL: SDL_Scancode = SDL_Scancode::CALL;
/// Used for rejecting phone calls.
pub const SDL_SCANCODE_ENDCALL: SDL_Scancode = SDL_Scancode::ENDCALL;
/// 400-500 reserved for dynamic keycodes
pub const SDL_SCANCODE_RESERVED: SDL_Scancode = SDL_Scancode::RESERVED;
/// not a key, just marks the number of scancodes for array bounds
pub const SDL_SCANCODE_COUNT: SDL_Scancode = SDL_Scancode::COUNT;
