use std::fmt::Display;

const KEY_RESERVED: u16 = 0;
const KEY_ESC: u16 = 1;
const KEY_1: u16 = 2;
const KEY_2: u16 = 3;
const KEY_3: u16 = 4;
const KEY_4: u16 = 5;
const KEY_5: u16 = 6;
const KEY_6: u16 = 7;
const KEY_7: u16 = 8;
const KEY_8: u16 = 9;
const KEY_9: u16 = 10;
const KEY_0: u16 = 11;
const KEY_MINUS: u16 = 12;
const KEY_EQUAL: u16 = 13;
const KEY_BACKSPACE: u16 = 14;
const KEY_TAB: u16 = 15;
const KEY_Q: u16 = 16;
const KEY_W: u16 = 17;
const KEY_E: u16 = 18;
const KEY_R: u16 = 19;
const KEY_T: u16 = 20;
const KEY_Y: u16 = 21;
const KEY_U: u16 = 22;
const KEY_I: u16 = 23;
const KEY_O: u16 = 24;
const KEY_P: u16 = 25;
const KEY_LEFTBRACE: u16 = 26;
const KEY_RIGHTBRACE: u16 = 27;
const KEY_ENTER: u16 = 28;
const KEY_LEFTCTRL: u16 = 29;
const KEY_A: u16 = 30;
const KEY_S: u16 = 31;
const KEY_D: u16 = 32;
const KEY_F: u16 = 33;
const KEY_G: u16 = 34;
const KEY_H: u16 = 35;
const KEY_J: u16 = 36;
const KEY_K: u16 = 37;
const KEY_L: u16 = 38;
const KEY_SEMICOLON: u16 = 39;
const KEY_APOSTROPHE: u16 = 40;
const KEY_GRAVE: u16 = 41;
const KEY_LEFTSHIFT: u16 = 42;
const KEY_BACKSLASH: u16 = 43;
const KEY_Z: u16 = 44;
const KEY_X: u16 = 45;
const KEY_C: u16 = 46;
const KEY_V: u16 = 47;
const KEY_B: u16 = 48;
const KEY_N: u16 = 49;
const KEY_M: u16 = 50;
const KEY_COMMA: u16 = 51;
const KEY_DOT: u16 = 52;
const KEY_SLASH: u16 = 53;
const KEY_RIGHTSHIFT: u16 = 54;
const KEY_KPASTERISK: u16 = 55;
const KEY_LEFTALT: u16 = 56;
const KEY_SPACE: u16 = 57;
const KEY_CAPSLOCK: u16 = 58;
const KEY_F1: u16 = 59;
const KEY_F2: u16 = 60;
const KEY_F3: u16 = 61;
const KEY_F4: u16 = 62;
const KEY_F5: u16 = 63;
const KEY_F6: u16 = 64;
const KEY_F7: u16 = 65;
const KEY_F8: u16 = 66;
const KEY_F9: u16 = 67;
const KEY_F10: u16 = 68;
const KEY_NUMLOCK: u16 = 69;
const KEY_SCROLLLOCK: u16 = 70;
const KEY_KP7: u16 = 71;
const KEY_KP8: u16 = 72;
const KEY_KP9: u16 = 73;
const KEY_KPMINUS: u16 = 74;
const KEY_KP4: u16 = 75;
const KEY_KP5: u16 = 76;
const KEY_KP6: u16 = 77;
const KEY_KPPLUS: u16 = 78;
const KEY_KP1: u16 = 79;
const KEY_KP2: u16 = 80;
const KEY_KP3: u16 = 81;
const KEY_KP0: u16 = 82;
const KEY_KPDOT: u16 = 83;

const KEY_ZENKAKUHANKAKU: u16 = 85;
const KEY_102ND: u16 = 86;
const KEY_F11: u16 = 87;
const KEY_F12: u16 = 88;
const KEY_RO: u16 = 89;
const KEY_KATAKANA: u16 = 90;
const KEY_HIRAGANA: u16 = 91;
const KEY_HENKAN: u16 = 92;
const KEY_KATAKANAHIRAGANA: u16 = 93;
const KEY_MUHENKAN: u16 = 94;
const KEY_KPJPCOMMA: u16 = 95;
const KEY_KPENTER: u16 = 96;
const KEY_RIGHTCTRL: u16 = 97;
const KEY_KPSLASH: u16 = 98;
const KEY_SYSRQ: u16 = 99;
const KEY_RIGHTALT: u16 = 100;
const KEY_LINEFEED: u16 = 101;
const KEY_HOME: u16 = 102;
const KEY_UP: u16 = 103;
const KEY_PAGEUP: u16 = 104;
const KEY_LEFT: u16 = 105;
const KEY_RIGHT: u16 = 106;
const KEY_END: u16 = 107;
const KEY_DOWN: u16 = 108;
const KEY_PAGEDOWN: u16 = 109;
const KEY_INSERT: u16 = 110;
const KEY_DELETE: u16 = 111;
const KEY_MACRO: u16 = 112;
const KEY_MUTE: u16 = 113;
const KEY_VOLUMEDOWN: u16 = 114;
const KEY_VOLUMEUP: u16 = 115;
const KEY_POWER: u16 = 116;
const KEY_KPEQUAL: u16 = 117;
const KEY_KPPLUSMINUS: u16 = 118;
const KEY_PAUSE: u16 = 119;
const KEY_SCALE: u16 = 120;

const KEY_KPCOMMA: u16 = 121;
const KEY_HANGEUL: u16 = 122;
const KEY_HANGUEL: u16 = 122;
const KEY_HANJA: u16 = 123;
const KEY_YEN: u16 = 124;
const KEY_LEFTMETA: u16 = 125;
const KEY_RIGHTMETA: u16 = 126;
const KEY_COMPOSE: u16 = 127;

const KEY_STOP: u16 = 128;
const KEY_AGAIN: u16 = 129;
const KEY_PROPS: u16 = 130;
const KEY_UNDO: u16 = 131;
const KEY_FRONT: u16 = 132;
const KEY_COPY: u16 = 133;
const KEY_OPEN: u16 = 134;
const KEY_PASTE: u16 = 135;
const KEY_FIND: u16 = 136;
const KEY_CUT: u16 = 137;
const KEY_HELP: u16 = 138;
const KEY_MENU: u16 = 139;
const KEY_CALC: u16 = 140;
const KEY_SETUP: u16 = 141;
const KEY_SLEEP: u16 = 142;
const KEY_WAKEUP: u16 = 143;
const KEY_FILE: u16 = 144;
const KEY_SENDFILE: u16 = 145;
const KEY_DELETEFILE: u16 = 146;
const KEY_XFER: u16 = 147;
const KEY_PROG1: u16 = 148;
const KEY_PROG2: u16 = 149;
const KEY_WWW: u16 = 150;
const KEY_MSDOS: u16 = 151;
const KEY_COFFEE: u16 = 152;
const KEY_SCREENLOCK: u16 = 152;
const KEY_ROTATE_DISPLAY: u16 = 153;
const KEY_DIRECTION: u16 = 153;
const KEY_CYCLEWINDOWS: u16 = 154;
const KEY_MAIL: u16 = 155;
const KEY_BOOKMARKS: u16 = 156;
const KEY_COMPUTER: u16 = 157;
const KEY_BACK: u16 = 158;
const KEY_FORWARD: u16 = 159;
const KEY_CLOSECD: u16 = 160;
const KEY_EJECTCD: u16 = 161;
const KEY_EJECTCLOSECD: u16 = 162;
const KEY_NEXTSONG: u16 = 163;
const KEY_PLAYPAUSE: u16 = 164;
const KEY_PREVIOUSSONG: u16 = 165;
const KEY_STOPCD: u16 = 166;
const KEY_RECORD: u16 = 167;
const KEY_REWIND: u16 = 168;
const KEY_PHONE: u16 = 169;
const KEY_ISO: u16 = 170;
const KEY_CONFIG: u16 = 171;
const KEY_HOMEPAGE: u16 = 172;
const KEY_REFRESH: u16 = 173;
const KEY_EXIT: u16 = 174;
const KEY_MOVE: u16 = 175;
const KEY_EDIT: u16 = 176;
const KEY_SCROLLUP: u16 = 177;
const KEY_SCROLLDOWN: u16 = 178;
const KEY_KPLEFTPAREN: u16 = 179;
const KEY_KPRIGHTPAREN: u16 = 180;
const KEY_NEW: u16 = 181;
const KEY_REDO: u16 = 182;

const KEY_F13: u16 = 183;
const KEY_F14: u16 = 184;
const KEY_F15: u16 = 185;
const KEY_F16: u16 = 186;
const KEY_F17: u16 = 187;
const KEY_F18: u16 = 188;
const KEY_F19: u16 = 189;
const KEY_F20: u16 = 190;
const KEY_F21: u16 = 191;
const KEY_F22: u16 = 192;
const KEY_F23: u16 = 193;
const KEY_F24: u16 = 194;

const KEY_PLAYCD: u16 = 200;
const KEY_PAUSECD: u16 = 201;
const KEY_PROG3: u16 = 202;
const KEY_PROG4: u16 = 203;
const KEY_ALL_APPLICATIONS: u16 = 204;
const KEY_DASHBOARD: u16 = 204;
const KEY_SUSPEND: u16 = 205;
const KEY_CLOSE: u16 = 206;
const KEY_PLAY: u16 = 207;
const KEY_FASTFORWARD: u16 = 208;
const KEY_BASSBOOST: u16 = 209;
const KEY_PRINT: u16 = 210;
const KEY_HP: u16 = 211;
const KEY_CAMERA: u16 = 212;
const KEY_SOUND: u16 = 213;
const KEY_QUESTION: u16 = 214;
const KEY_EMAIL: u16 = 215;
const KEY_CHAT: u16 = 216;
const KEY_SEARCH: u16 = 217;
const KEY_CONNECT: u16 = 218;
const KEY_FINANCE: u16 = 219;
const KEY_SPORT: u16 = 220;
const KEY_SHOP: u16 = 221;
const KEY_ALTERASE: u16 = 222;
const KEY_CANCEL: u16 = 223;
const KEY_BRIGHTNESSDOWN: u16 = 224;
const KEY_BRIGHTNESSUP: u16 = 225;
const KEY_MEDIA: u16 = 226;

const KEY_SWITCHVIDEOMODE: u16 = 227;
const KEY_KBDILLUMTOGGLE: u16 = 228;
const KEY_KBDILLUMDOWN: u16 = 229;
const KEY_KBDILLUMUP: u16 = 230;

const KEY_SEND: u16 = 231;
const KEY_REPLY: u16 = 232;
const KEY_FORWARDMAIL: u16 = 233;
const KEY_SAVE: u16 = 234;
const KEY_DOCUMENTS: u16 = 235;

const KEY_BATTERY: u16 = 236;

const KEY_BLUETOOTH: u16 = 237;
const KEY_WLAN: u16 = 238;
const KEY_UWB: u16 = 239;

const KEY_UNKNOWN: u16 = 240;

const KEY_VIDEO_NEXT: u16 = 241;
const KEY_VIDEO_PREV: u16 = 242;
const KEY_BRIGHTNESS_CYCLE: u16 = 243;
const KEY_BRIGHTNESS_AUTO: u16 = 244;

const KEY_BRIGHTNESS_ZERO: u16 = 244;
const KEY_DISPLAY_OFF: u16 = 245;

const KEY_WWAN: u16 = 246;
const KEY_WIMAX: u16 = 246;
const KEY_RFKILL: u16 = 247;

const KEY_MICMUTE: u16 = 248;

#[derive(Clone, Copy, Debug)]
pub enum KeyCode {
    KEY_RESERVED,
    KEY_ESC,
    KEY_1,
    KEY_2,
    KEY_3,
    KEY_4,
    KEY_5,
    KEY_6,
    KEY_7,
    KEY_8,
    KEY_9,
    KEY_0,
    KEY_MINUS,
    KEY_EQUAL,
    KEY_BACKSPACE,
    KEY_TAB,
    KEY_Q,
    KEY_W,
    KEY_E,
    KEY_R,
    KEY_T,
    KEY_Y,
    KEY_U,
    KEY_I,
    KEY_O,
    KEY_P,
    KEY_LEFTBRACE,
    KEY_RIGHTBRACE,
    KEY_ENTER,
    KEY_LEFTCTRL,
    KEY_A,
    KEY_S,
    KEY_D,
    KEY_F,
    KEY_G,
    KEY_H,
    KEY_J,
    KEY_K,
    KEY_L,
    KEY_SEMICOLON,
    KEY_APOSTROPHE,
    KEY_GRAVE,
    KEY_LEFTSHIFT,
    KEY_BACKSLASH,
    KEY_Z,
    KEY_X,
    KEY_C,
    KEY_V,
    KEY_B,
    KEY_N,
    KEY_M,
    KEY_COMMA,
    KEY_DOT,
    KEY_SLASH,
    KEY_RIGHTSHIFT,
    KEY_KPASTERISK,
    KEY_LEFTALT,
    KEY_SPACE,
    KEY_CAPSLOCK,
    KEY_F1,
    KEY_F2,
    KEY_F3,
    KEY_F4,
    KEY_F5,
    KEY_F6,
    KEY_F7,
    KEY_F8,
    KEY_F9,
    KEY_F10,
    KEY_NUMLOCK,
    KEY_SCROLLLOCK,
    KEY_KP7,
    KEY_KP8,
    KEY_KP9,
    KEY_KPMINUS,
    KEY_KP4,
    KEY_KP5,
    KEY_KP6,
    KEY_KPPLUS,
    KEY_KP1,
    KEY_KP2,
    KEY_KP3,
    KEY_KP0,
    KEY_KPDOT,

    KEY_ZENKAKUHANKAKU,
    KEY_102ND,
    KEY_F11,
    KEY_F12,
    KEY_RO,
    KEY_KATAKANA,
    KEY_HIRAGANA,
    KEY_HENKAN,
    KEY_KATAKANAHIRAGANA,
    KEY_MUHENKAN,
    KEY_KPJPCOMMA,
    KEY_KPENTER,
    KEY_RIGHTCTRL,
    KEY_KPSLASH,
    KEY_SYSRQ,
    KEY_RIGHTALT,
    KEY_LINEFEED,
    KEY_HOME,
    KEY_UP,
    KEY_PAGEUP,
    KEY_LEFT,
    KEY_RIGHT,
    KEY_END,
    KEY_DOWN,
    KEY_PAGEDOWN,
    KEY_INSERT,
    KEY_DELETE,
    KEY_MACRO,
    KEY_MUTE,
    KEY_VOLUMEDOWN,
    KEY_VOLUMEUP,
    KEY_POWER,
    KEY_KPEQUAL,
    KEY_KPPLUSMINUS,
    KEY_PAUSE,
    KEY_SCALE,

    KEY_KPCOMMA,
    KEY_HANGEUL,
    KEY_HANGUEL,
    KEY_HANJA,
    KEY_YEN,
    KEY_LEFTMETA,
    KEY_RIGHTMETA,
    KEY_COMPOSE,

    KEY_STOP,
    KEY_AGAIN,
    KEY_PROPS,
    KEY_UNDO,
    KEY_FRONT,
    KEY_COPY,
    KEY_OPEN,
    KEY_PASTE,
    KEY_FIND,
    KEY_CUT,
    KEY_HELP,
    KEY_MENU,
    KEY_CALC,
    KEY_SETUP,
    KEY_SLEEP,
    KEY_WAKEUP,
    KEY_FILE,
    KEY_SENDFILE,
    KEY_DELETEFILE,
    KEY_XFER,
    KEY_PROG1,
    KEY_PROG2,
    KEY_WWW,
    KEY_MSDOS,
    KEY_COFFEE,
    KEY_SCREENLOCK,
    KEY_ROTATE_DISPLAY,
    KEY_DIRECTION,
    KEY_CYCLEWINDOWS,
    KEY_MAIL,
    KEY_BOOKMARKS,
    KEY_COMPUTER,
    KEY_BACK,
    KEY_FORWARD,
    KEY_CLOSECD,
    KEY_EJECTCD,
    KEY_EJECTCLOSECD,
    KEY_NEXTSONG,
    KEY_PLAYPAUSE,
    KEY_PREVIOUSSONG,
    KEY_STOPCD,
    KEY_RECORD,
    KEY_REWIND,
    KEY_PHONE,
    KEY_ISO,
    KEY_CONFIG,
    KEY_HOMEPAGE,
    KEY_REFRESH,
    KEY_EXIT,
    KEY_MOVE,
    KEY_EDIT,
    KEY_SCROLLUP,
    KEY_SCROLLDOWN,
    KEY_KPLEFTPAREN,
    KEY_KPRIGHTPAREN,
    KEY_NEW,
    KEY_REDO,

    KEY_F13,
    KEY_F14,
    KEY_F15,
    KEY_F16,
    KEY_F17,
    KEY_F18,
    KEY_F19,
    KEY_F20,
    KEY_F21,
    KEY_F22,
    KEY_F23,
    KEY_F24,

    KEY_PLAYCD,
    KEY_PAUSECD,
    KEY_PROG3,
    KEY_PROG4,
    KEY_ALL_APPLICATIONS,
    KEY_DASHBOARD,
    KEY_SUSPEND,
    KEY_CLOSE,
    KEY_PLAY,
    KEY_FASTFORWARD,
    KEY_BASSBOOST,
    KEY_PRINT,
    KEY_HP,
    KEY_CAMERA,
    KEY_SOUND,
    KEY_QUESTION,
    KEY_EMAIL,
    KEY_CHAT,
    KEY_SEARCH,
    KEY_CONNECT,
    KEY_FINANCE,
    KEY_SPORT,
    KEY_SHOP,
    KEY_ALTERASE,
    KEY_CANCEL,
    KEY_BRIGHTNESSDOWN,
    KEY_BRIGHTNESSUP,
    KEY_MEDIA,

    KEY_SWITCHVIDEOMODE,
    KEY_KBDILLUMTOGGLE,
    KEY_KBDILLUMDOWN,
    KEY_KBDILLUMUP,

    KEY_SEND,
    KEY_REPLY,
    KEY_FORWARDMAIL,
    KEY_SAVE,
    KEY_DOCUMENTS,

    KEY_BATTERY,

    KEY_BLUETOOTH,
    KEY_WLAN,
    KEY_UWB,

    KEY_UNKNOWN,

    KEY_VIDEO_NEXT,
    KEY_VIDEO_PREV,
    KEY_BRIGHTNESS_CYCLE,
    KEY_BRIGHTNESS_AUTO,

    KEY_BRIGHTNESS_ZERO,
    KEY_DISPLAY_OFF,

    KEY_WWAN,
    KEY_WIMAX,
    KEY_RFKILL,

    KEY_MICMUTE,
}

impl KeyCode {
    pub fn to_value(&self) -> u16 {
        match self {
            KeyCode::KEY_RESERVED => KEY_RESERVED,
            KeyCode::KEY_ESC => KEY_ESC,
            KeyCode::KEY_1 => KEY_1,
            KeyCode::KEY_2 => KEY_2,
            KeyCode::KEY_3 => KEY_3,
            KeyCode::KEY_4 => KEY_4,
            KeyCode::KEY_5 => KEY_5,
            KeyCode::KEY_6 => KEY_6,
            KeyCode::KEY_7 => KEY_7,
            KeyCode::KEY_8 => KEY_8,
            KeyCode::KEY_9 => KEY_9,
            KeyCode::KEY_0 => KEY_0,
            KeyCode::KEY_MINUS => KEY_MINUS,
            KeyCode::KEY_EQUAL => KEY_EQUAL,
            KeyCode::KEY_BACKSPACE => KEY_BACKSPACE,
            KeyCode::KEY_TAB => KEY_TAB,
            KeyCode::KEY_Q => KEY_Q,
            KeyCode::KEY_W => KEY_W,
            KeyCode::KEY_E => KEY_E,
            KeyCode::KEY_R => KEY_R,
            KeyCode::KEY_T => KEY_T,
            KeyCode::KEY_Y => KEY_Y,
            KeyCode::KEY_U => KEY_U,
            KeyCode::KEY_I => KEY_I,
            KeyCode::KEY_O => KEY_O,
            KeyCode::KEY_P => KEY_P,
            KeyCode::KEY_LEFTBRACE => KEY_LEFTBRACE,
            KeyCode::KEY_RIGHTBRACE => KEY_RIGHTBRACE,
            KeyCode::KEY_ENTER => KEY_ENTER,
            KeyCode::KEY_LEFTCTRL => KEY_LEFTCTRL,
            KeyCode::KEY_A => KEY_A,
            KeyCode::KEY_S => KEY_S,
            KeyCode::KEY_D => KEY_D,
            KeyCode::KEY_F => KEY_F,
            KeyCode::KEY_G => KEY_G,
            KeyCode::KEY_H => KEY_H,
            KeyCode::KEY_J => KEY_J,
            KeyCode::KEY_K => KEY_K,
            KeyCode::KEY_L => KEY_L,
            KeyCode::KEY_SEMICOLON => KEY_SEMICOLON,
            KeyCode::KEY_APOSTROPHE => KEY_APOSTROPHE,
            KeyCode::KEY_GRAVE => KEY_GRAVE,
            KeyCode::KEY_LEFTSHIFT => KEY_LEFTSHIFT,
            KeyCode::KEY_BACKSLASH => KEY_BACKSLASH,
            KeyCode::KEY_Z => KEY_Z,
            KeyCode::KEY_X => KEY_X,
            KeyCode::KEY_C => KEY_C,
            KeyCode::KEY_V => KEY_V,
            KeyCode::KEY_B => KEY_B,
            KeyCode::KEY_N => KEY_N,
            KeyCode::KEY_M => KEY_M,
            KeyCode::KEY_COMMA => KEY_COMMA,
            KeyCode::KEY_DOT => KEY_DOT,
            KeyCode::KEY_SLASH => KEY_SLASH,
            KeyCode::KEY_RIGHTSHIFT => KEY_RIGHTSHIFT,
            KeyCode::KEY_KPASTERISK => KEY_KPASTERISK,
            KeyCode::KEY_LEFTALT => KEY_LEFTALT,
            KeyCode::KEY_SPACE => KEY_SPACE,
            KeyCode::KEY_CAPSLOCK => KEY_CAPSLOCK,
            KeyCode::KEY_F1 => KEY_F1,
            KeyCode::KEY_F2 => KEY_F2,
            KeyCode::KEY_F3 => KEY_F3,
            KeyCode::KEY_F4 => KEY_F4,
            KeyCode::KEY_F5 => KEY_F5,
            KeyCode::KEY_F6 => KEY_F6,
            KeyCode::KEY_F7 => KEY_F7,
            KeyCode::KEY_F8 => KEY_F8,
            KeyCode::KEY_F9 => KEY_F9,
            KeyCode::KEY_F10 => KEY_F10,
            KeyCode::KEY_NUMLOCK => KEY_NUMLOCK,
            KeyCode::KEY_SCROLLLOCK => KEY_SCROLLLOCK,
            KeyCode::KEY_KP7 => KEY_KP7,
            KeyCode::KEY_KP8 => KEY_KP8,
            KeyCode::KEY_KP9 => KEY_KP9,
            KeyCode::KEY_KPMINUS => KEY_KPMINUS,
            KeyCode::KEY_KP4 => KEY_KP4,
            KeyCode::KEY_KP5 => KEY_KP5,
            KeyCode::KEY_KP6 => KEY_KP6,
            KeyCode::KEY_KPPLUS => KEY_KPPLUS,
            KeyCode::KEY_KP1 => KEY_KP1,
            KeyCode::KEY_KP2 => KEY_KP2,
            KeyCode::KEY_KP3 => KEY_KP3,
            KeyCode::KEY_KP0 => KEY_KP0,
            KeyCode::KEY_KPDOT => KEY_KPDOT,

            KeyCode::KEY_ZENKAKUHANKAKU => KEY_ZENKAKUHANKAKU,
            KeyCode::KEY_102ND => KEY_102ND,
            KeyCode::KEY_F11 => KEY_F11,
            KeyCode::KEY_F12 => KEY_F12,
            KeyCode::KEY_RO => KEY_RO,
            KeyCode::KEY_KATAKANA => KEY_KATAKANA,
            KeyCode::KEY_HIRAGANA => KEY_HIRAGANA,
            KeyCode::KEY_HENKAN => KEY_HENKAN,
            KeyCode::KEY_KATAKANAHIRAGANA => KEY_KATAKANAHIRAGANA,
            KeyCode::KEY_MUHENKAN => KEY_MUHENKAN,
            KeyCode::KEY_KPJPCOMMA => KEY_KPJPCOMMA,
            KeyCode::KEY_KPENTER => KEY_KPENTER,
            KeyCode::KEY_RIGHTCTRL => KEY_RIGHTCTRL,
            KeyCode::KEY_KPSLASH => KEY_KPSLASH,
            KeyCode::KEY_SYSRQ => KEY_SYSRQ,
            KeyCode::KEY_RIGHTALT => KEY_RIGHTALT,
            KeyCode::KEY_LINEFEED => KEY_LINEFEED,
            KeyCode::KEY_HOME => KEY_HOME,
            KeyCode::KEY_UP => KEY_UP,
            KeyCode::KEY_PAGEUP => KEY_PAGEUP,
            KeyCode::KEY_LEFT => KEY_LEFT,
            KeyCode::KEY_RIGHT => KEY_RIGHT,
            KeyCode::KEY_END => KEY_END,
            KeyCode::KEY_DOWN => KEY_DOWN,
            KeyCode::KEY_PAGEDOWN => KEY_PAGEDOWN,
            KeyCode::KEY_INSERT => KEY_INSERT,
            KeyCode::KEY_DELETE => KEY_DELETE,
            KeyCode::KEY_MACRO => KEY_MACRO,
            KeyCode::KEY_MUTE => KEY_MUTE,
            KeyCode::KEY_VOLUMEDOWN => KEY_VOLUMEDOWN,
            KeyCode::KEY_VOLUMEUP => KEY_VOLUMEUP,
            KeyCode::KEY_POWER => KEY_POWER,
            KeyCode::KEY_KPEQUAL => KEY_KPEQUAL,
            KeyCode::KEY_KPPLUSMINUS => KEY_KPPLUSMINUS,
            KeyCode::KEY_PAUSE => KEY_PAUSE,
            KeyCode::KEY_SCALE => KEY_SCALE,

            KeyCode::KEY_KPCOMMA => KEY_KPCOMMA,
            KeyCode::KEY_HANGEUL => KEY_HANGEUL,
            KeyCode::KEY_HANGUEL => KEY_HANGUEL,
            KeyCode::KEY_HANJA => KEY_HANJA,
            KeyCode::KEY_YEN => KEY_YEN,
            KeyCode::KEY_LEFTMETA => KEY_LEFTMETA,
            KeyCode::KEY_RIGHTMETA => KEY_RIGHTMETA,
            KeyCode::KEY_COMPOSE => KEY_COMPOSE,

            KeyCode::KEY_STOP => KEY_STOP,
            KeyCode::KEY_AGAIN => KEY_AGAIN,
            KeyCode::KEY_PROPS => KEY_PROPS,
            KeyCode::KEY_UNDO => KEY_UNDO,
            KeyCode::KEY_FRONT => KEY_FRONT,
            KeyCode::KEY_COPY => KEY_COPY,
            KeyCode::KEY_OPEN => KEY_OPEN,
            KeyCode::KEY_PASTE => KEY_PASTE,
            KeyCode::KEY_FIND => KEY_FIND,
            KeyCode::KEY_CUT => KEY_CUT,
            KeyCode::KEY_HELP => KEY_HELP,
            KeyCode::KEY_MENU => KEY_MENU,
            KeyCode::KEY_CALC => KEY_CALC,
            KeyCode::KEY_SETUP => KEY_SETUP,
            KeyCode::KEY_SLEEP => KEY_SLEEP,
            KeyCode::KEY_WAKEUP => KEY_WAKEUP,
            KeyCode::KEY_FILE => KEY_FILE,
            KeyCode::KEY_SENDFILE => KEY_SENDFILE,
            KeyCode::KEY_DELETEFILE => KEY_DELETEFILE,
            KeyCode::KEY_XFER => KEY_XFER,
            KeyCode::KEY_PROG1 => KEY_PROG1,
            KeyCode::KEY_PROG2 => KEY_PROG2,
            KeyCode::KEY_WWW => KEY_WWW,
            KeyCode::KEY_MSDOS => KEY_MSDOS,
            KeyCode::KEY_COFFEE => KEY_COFFEE,
            KeyCode::KEY_SCREENLOCK => KEY_SCREENLOCK,
            KeyCode::KEY_ROTATE_DISPLAY => KEY_ROTATE_DISPLAY,
            KeyCode::KEY_DIRECTION => KEY_DIRECTION,
            KeyCode::KEY_CYCLEWINDOWS => KEY_CYCLEWINDOWS,
            KeyCode::KEY_MAIL => KEY_MAIL,
            KeyCode::KEY_BOOKMARKS => KEY_BOOKMARKS,
            KeyCode::KEY_COMPUTER => KEY_COMPUTER,
            KeyCode::KEY_BACK => KEY_BACK,
            KeyCode::KEY_FORWARD => KEY_FORWARD,
            KeyCode::KEY_CLOSECD => KEY_CLOSECD,
            KeyCode::KEY_EJECTCD => KEY_EJECTCD,
            KeyCode::KEY_EJECTCLOSECD => KEY_EJECTCLOSECD,
            KeyCode::KEY_NEXTSONG => KEY_NEXTSONG,
            KeyCode::KEY_PLAYPAUSE => KEY_PLAYPAUSE,
            KeyCode::KEY_PREVIOUSSONG => KEY_PREVIOUSSONG,
            KeyCode::KEY_STOPCD => KEY_STOPCD,
            KeyCode::KEY_RECORD => KEY_RECORD,
            KeyCode::KEY_REWIND => KEY_REWIND,
            KeyCode::KEY_PHONE => KEY_PHONE,
            KeyCode::KEY_ISO => KEY_ISO,
            KeyCode::KEY_CONFIG => KEY_CONFIG,
            KeyCode::KEY_HOMEPAGE => KEY_HOMEPAGE,
            KeyCode::KEY_REFRESH => KEY_REFRESH,
            KeyCode::KEY_EXIT => KEY_EXIT,
            KeyCode::KEY_MOVE => KEY_MOVE,
            KeyCode::KEY_EDIT => KEY_EDIT,
            KeyCode::KEY_SCROLLUP => KEY_SCROLLUP,
            KeyCode::KEY_SCROLLDOWN => KEY_SCROLLDOWN,
            KeyCode::KEY_KPLEFTPAREN => KEY_KPLEFTPAREN,
            KeyCode::KEY_KPRIGHTPAREN => KEY_KPRIGHTPAREN,
            KeyCode::KEY_NEW => KEY_NEW,
            KeyCode::KEY_REDO => KEY_REDO,

            KeyCode::KEY_F13 => KEY_F13,
            KeyCode::KEY_F14 => KEY_F14,
            KeyCode::KEY_F15 => KEY_F15,
            KeyCode::KEY_F16 => KEY_F16,
            KeyCode::KEY_F17 => KEY_F17,
            KeyCode::KEY_F18 => KEY_F18,
            KeyCode::KEY_F19 => KEY_F19,
            KeyCode::KEY_F20 => KEY_F20,
            KeyCode::KEY_F21 => KEY_F21,
            KeyCode::KEY_F22 => KEY_F22,
            KeyCode::KEY_F23 => KEY_F23,
            KeyCode::KEY_F24 => KEY_F24,

            KeyCode::KEY_PLAYCD => KEY_PLAYCD,
            KeyCode::KEY_PAUSECD => KEY_PAUSECD,
            KeyCode::KEY_PROG3 => KEY_PROG3,
            KeyCode::KEY_PROG4 => KEY_PROG4,
            KeyCode::KEY_ALL_APPLICATIONS => KEY_ALL_APPLICATIONS,
            KeyCode::KEY_DASHBOARD => KEY_DASHBOARD,
            KeyCode::KEY_SUSPEND => KEY_SUSPEND,
            KeyCode::KEY_CLOSE => KEY_CLOSE,
            KeyCode::KEY_PLAY => KEY_PLAY,
            KeyCode::KEY_FASTFORWARD => KEY_FASTFORWARD,
            KeyCode::KEY_BASSBOOST => KEY_BASSBOOST,
            KeyCode::KEY_PRINT => KEY_PRINT,
            KeyCode::KEY_HP => KEY_HP,
            KeyCode::KEY_CAMERA => KEY_CAMERA,
            KeyCode::KEY_SOUND => KEY_SOUND,
            KeyCode::KEY_QUESTION => KEY_QUESTION,
            KeyCode::KEY_EMAIL => KEY_EMAIL,
            KeyCode::KEY_CHAT => KEY_CHAT,
            KeyCode::KEY_SEARCH => KEY_SEARCH,
            KeyCode::KEY_CONNECT => KEY_CONNECT,
            KeyCode::KEY_FINANCE => KEY_FINANCE,
            KeyCode::KEY_SPORT => KEY_SPORT,
            KeyCode::KEY_SHOP => KEY_SHOP,
            KeyCode::KEY_ALTERASE => KEY_ALTERASE,
            KeyCode::KEY_CANCEL => KEY_CANCEL,
            KeyCode::KEY_BRIGHTNESSDOWN => KEY_BRIGHTNESSDOWN,
            KeyCode::KEY_BRIGHTNESSUP => KEY_BRIGHTNESSUP,
            KeyCode::KEY_MEDIA => KEY_MEDIA,

            KeyCode::KEY_SWITCHVIDEOMODE => KEY_SWITCHVIDEOMODE,
            KeyCode::KEY_KBDILLUMTOGGLE => KEY_KBDILLUMTOGGLE,
            KeyCode::KEY_KBDILLUMDOWN => KEY_KBDILLUMDOWN,
            KeyCode::KEY_KBDILLUMUP => KEY_KBDILLUMUP,

            KeyCode::KEY_SEND => KEY_SEND,
            KeyCode::KEY_REPLY => KEY_REPLY,
            KeyCode::KEY_FORWARDMAIL => KEY_FORWARDMAIL,
            KeyCode::KEY_SAVE => KEY_SAVE,
            KeyCode::KEY_DOCUMENTS => KEY_DOCUMENTS,

            KeyCode::KEY_BATTERY => KEY_BATTERY,

            KeyCode::KEY_BLUETOOTH => KEY_BLUETOOTH,
            KeyCode::KEY_WLAN => KEY_WLAN,
            KeyCode::KEY_UWB => KEY_UWB,

            KeyCode::KEY_UNKNOWN => KEY_UNKNOWN,

            KeyCode::KEY_VIDEO_NEXT => KEY_VIDEO_NEXT,
            KeyCode::KEY_VIDEO_PREV => KEY_VIDEO_PREV,
            KeyCode::KEY_BRIGHTNESS_CYCLE => KEY_BRIGHTNESS_CYCLE,
            KeyCode::KEY_BRIGHTNESS_AUTO => KEY_BRIGHTNESS_AUTO,

            KeyCode::KEY_BRIGHTNESS_ZERO => KEY_BRIGHTNESS_ZERO,
            KeyCode::KEY_DISPLAY_OFF => KEY_DISPLAY_OFF,

            KeyCode::KEY_WWAN => KEY_WWAN,
            KeyCode::KEY_WIMAX => KEY_WIMAX,
            KeyCode::KEY_RFKILL => KEY_RFKILL,

            KeyCode::KEY_MICMUTE => KEY_MICMUTE,
        }
    }
    pub fn from_value(value: u16) -> Self {
        match value {
            KEY_ESC => KeyCode::KEY_ESC,
            KEY_1 => KeyCode::KEY_1,
            KEY_2 => KeyCode::KEY_2,
            KEY_3 => KeyCode::KEY_3,
            KEY_4 => KeyCode::KEY_4,
            KEY_5 => KeyCode::KEY_5,
            KEY_6 => KeyCode::KEY_6,
            KEY_7 => KeyCode::KEY_7,
            KEY_8 => KeyCode::KEY_8,
            KEY_9 => KeyCode::KEY_9,
            KEY_0 => KeyCode::KEY_0,
            KEY_MINUS => KeyCode::KEY_MINUS,
            KEY_EQUAL => KeyCode::KEY_EQUAL,
            KEY_BACKSPACE => KeyCode::KEY_BACKSPACE,
            KEY_TAB => KeyCode::KEY_TAB,
            KEY_Q => KeyCode::KEY_Q,
            KEY_W => KeyCode::KEY_W,
            KEY_E => KeyCode::KEY_E,
            KEY_R => KeyCode::KEY_R,
            KEY_T => KeyCode::KEY_T,
            KEY_Y => KeyCode::KEY_Y,
            KEY_U => KeyCode::KEY_U,
            KEY_I => KeyCode::KEY_I,
            KEY_O => KeyCode::KEY_O,
            KEY_P => KeyCode::KEY_P,
            KEY_LEFTBRACE => KeyCode::KEY_LEFTBRACE,
            KEY_RIGHTBRACE => KeyCode::KEY_RIGHTBRACE,
            KEY_ENTER => KeyCode::KEY_ENTER,
            KEY_LEFTCTRL => KeyCode::KEY_LEFTCTRL,
            KEY_A => KeyCode::KEY_A,
            KEY_S => KeyCode::KEY_S,
            KEY_D => KeyCode::KEY_D,
            KEY_F => KeyCode::KEY_F,
            KEY_G => KeyCode::KEY_G,
            KEY_H => KeyCode::KEY_H,
            KEY_J => KeyCode::KEY_J,
            KEY_K => KeyCode::KEY_K,
            KEY_L => KeyCode::KEY_L,
            KEY_SEMICOLON => KeyCode::KEY_SEMICOLON,
            KEY_APOSTROPHE => KeyCode::KEY_APOSTROPHE,
            KEY_GRAVE => KeyCode::KEY_GRAVE,
            KEY_LEFTSHIFT => KeyCode::KEY_LEFTSHIFT,
            KEY_BACKSLASH => KeyCode::KEY_BACKSLASH,
            KEY_Z => KeyCode::KEY_Z,
            KEY_X => KeyCode::KEY_X,
            KEY_C => KeyCode::KEY_C,
            KEY_V => KeyCode::KEY_V,
            KEY_B => KeyCode::KEY_B,
            KEY_N => KeyCode::KEY_N,
            KEY_M => KeyCode::KEY_M,
            KEY_COMMA => KeyCode::KEY_COMMA,
            KEY_DOT => KeyCode::KEY_DOT,
            KEY_SLASH => KeyCode::KEY_SLASH,
            KEY_RIGHTSHIFT => KeyCode::KEY_RIGHTSHIFT,
            KEY_KPASTERISK => KeyCode::KEY_KPASTERISK,
            KEY_LEFTALT => KeyCode::KEY_LEFTALT,
            KEY_SPACE => KeyCode::KEY_SPACE,
            KEY_CAPSLOCK => KeyCode::KEY_CAPSLOCK,
            KEY_F1 => KeyCode::KEY_F1,
            KEY_F2 => KeyCode::KEY_F2,
            KEY_F3 => KeyCode::KEY_F3,
            KEY_F4 => KeyCode::KEY_F4,
            KEY_F5 => KeyCode::KEY_F5,
            KEY_F6 => KeyCode::KEY_F6,
            KEY_F7 => KeyCode::KEY_F7,
            KEY_F8 => KeyCode::KEY_F8,
            KEY_F9 => KeyCode::KEY_F9,
            KEY_F10 => KeyCode::KEY_F10,
            KEY_NUMLOCK => KeyCode::KEY_NUMLOCK,
            KEY_SCROLLLOCK => KeyCode::KEY_SCROLLLOCK,
            KEY_KP7 => KeyCode::KEY_KP7,
            KEY_KP8 => KeyCode::KEY_KP8,
            KEY_KP9 => KeyCode::KEY_KP9,
            KEY_KPMINUS => KeyCode::KEY_KPMINUS,
            KEY_KP4 => KeyCode::KEY_KP4,
            KEY_KP5 => KeyCode::KEY_KP5,
            KEY_KP6 => KeyCode::KEY_KP6,
            KEY_KPPLUS => KeyCode::KEY_KPPLUS,
            KEY_KP1 => KeyCode::KEY_KP1,
            KEY_KP2 => KeyCode::KEY_KP2,
            KEY_KP3 => KeyCode::KEY_KP3,
            KEY_KP0 => KeyCode::KEY_KP0,
            KEY_KPDOT => KeyCode::KEY_KPDOT,

            KEY_ZENKAKUHANKAKU => KeyCode::KEY_ZENKAKUHANKAKU,
            KEY_102ND => KeyCode::KEY_102ND,
            KEY_F11 => KeyCode::KEY_F11,
            KEY_F12 => KeyCode::KEY_F12,
            KEY_RO => KeyCode::KEY_RO,
            KEY_KATAKANA => KeyCode::KEY_KATAKANA,
            KEY_HIRAGANA => KeyCode::KEY_HIRAGANA,
            KEY_HENKAN => KeyCode::KEY_HENKAN,
            KEY_KATAKANAHIRAGANA => KeyCode::KEY_KATAKANAHIRAGANA,
            KEY_MUHENKAN => KeyCode::KEY_MUHENKAN,
            KEY_KPJPCOMMA => KeyCode::KEY_KPJPCOMMA,
            KEY_KPENTER => KeyCode::KEY_KPENTER,
            KEY_RIGHTCTRL => KeyCode::KEY_RIGHTCTRL,
            KEY_KPSLASH => KeyCode::KEY_KPSLASH,
            KEY_SYSRQ => KeyCode::KEY_SYSRQ,
            KEY_RIGHTALT => KeyCode::KEY_RIGHTALT,
            KEY_LINEFEED => KeyCode::KEY_LINEFEED,
            KEY_HOME => KeyCode::KEY_HOME,
            KEY_UP => KeyCode::KEY_UP,
            KEY_PAGEUP => KeyCode::KEY_PAGEUP,
            KEY_LEFT => KeyCode::KEY_LEFT,
            KEY_RIGHT => KeyCode::KEY_RIGHT,
            KEY_END => KeyCode::KEY_END,
            KEY_DOWN => KeyCode::KEY_DOWN,
            KEY_PAGEDOWN => KeyCode::KEY_PAGEDOWN,
            KEY_INSERT => KeyCode::KEY_INSERT,
            KEY_DELETE => KeyCode::KEY_DELETE,
            KEY_MACRO => KeyCode::KEY_MACRO,
            KEY_MUTE => KeyCode::KEY_MUTE,
            KEY_VOLUMEDOWN => KeyCode::KEY_VOLUMEDOWN,
            KEY_VOLUMEUP => KeyCode::KEY_VOLUMEUP,
            KEY_POWER => KeyCode::KEY_POWER,
            KEY_KPEQUAL => KeyCode::KEY_KPEQUAL,
            KEY_KPPLUSMINUS => KeyCode::KEY_KPPLUSMINUS,
            KEY_PAUSE => KeyCode::KEY_PAUSE,
            KEY_SCALE => KeyCode::KEY_SCALE,

            KEY_KPCOMMA => KeyCode::KEY_KPCOMMA,
            KEY_HANGEUL => KeyCode::KEY_HANGEUL,
            KEY_HANGUEL => KeyCode::KEY_HANGUEL,
            KEY_HANJA => KeyCode::KEY_HANJA,
            KEY_YEN => KeyCode::KEY_YEN,
            KEY_LEFTMETA => KeyCode::KEY_LEFTMETA,
            KEY_RIGHTMETA => KeyCode::KEY_RIGHTMETA,
            KEY_COMPOSE => KeyCode::KEY_COMPOSE,

            KEY_STOP => KeyCode::KEY_STOP,
            KEY_AGAIN => KeyCode::KEY_AGAIN,
            KEY_PROPS => KeyCode::KEY_PROPS,
            KEY_UNDO => KeyCode::KEY_UNDO,
            KEY_FRONT => KeyCode::KEY_FRONT,
            KEY_COPY => KeyCode::KEY_COPY,
            KEY_OPEN => KeyCode::KEY_OPEN,
            KEY_PASTE => KeyCode::KEY_PASTE,
            KEY_FIND => KeyCode::KEY_FIND,
            KEY_CUT => KeyCode::KEY_CUT,
            KEY_HELP => KeyCode::KEY_HELP,
            KEY_MENU => KeyCode::KEY_MENU,
            KEY_CALC => KeyCode::KEY_CALC,
            KEY_SETUP => KeyCode::KEY_SETUP,
            KEY_SLEEP => KeyCode::KEY_SLEEP,
            KEY_WAKEUP => KeyCode::KEY_WAKEUP,
            KEY_FILE => KeyCode::KEY_FILE,
            KEY_SENDFILE => KeyCode::KEY_SENDFILE,
            KEY_DELETEFILE => KeyCode::KEY_DELETEFILE,
            KEY_XFER => KeyCode::KEY_XFER,
            KEY_PROG1 => KeyCode::KEY_PROG1,
            KEY_PROG2 => KeyCode::KEY_PROG2,
            KEY_WWW => KeyCode::KEY_WWW,
            KEY_MSDOS => KeyCode::KEY_MSDOS,
            KEY_COFFEE => KeyCode::KEY_COFFEE,
            KEY_SCREENLOCK => KeyCode::KEY_SCREENLOCK,
            KEY_ROTATE_DISPLAY => KeyCode::KEY_ROTATE_DISPLAY,
            KEY_DIRECTION => KeyCode::KEY_DIRECTION,
            KEY_CYCLEWINDOWS => KeyCode::KEY_CYCLEWINDOWS,
            KEY_MAIL => KeyCode::KEY_MAIL,
            KEY_BOOKMARKS => KeyCode::KEY_BOOKMARKS,
            KEY_COMPUTER => KeyCode::KEY_COMPUTER,
            KEY_BACK => KeyCode::KEY_BACK,
            KEY_FORWARD => KeyCode::KEY_FORWARD,
            KEY_CLOSECD => KeyCode::KEY_CLOSECD,
            KEY_EJECTCD => KeyCode::KEY_EJECTCD,
            KEY_EJECTCLOSECD => KeyCode::KEY_EJECTCLOSECD,
            KEY_NEXTSONG => KeyCode::KEY_NEXTSONG,
            KEY_PLAYPAUSE => KeyCode::KEY_PLAYPAUSE,
            KEY_PREVIOUSSONG => KeyCode::KEY_PREVIOUSSONG,
            KEY_STOPCD => KeyCode::KEY_STOPCD,
            KEY_RECORD => KeyCode::KEY_RECORD,
            KEY_REWIND => KeyCode::KEY_REWIND,
            KEY_PHONE => KeyCode::KEY_PHONE,
            KEY_ISO => KeyCode::KEY_ISO,
            KEY_CONFIG => KeyCode::KEY_CONFIG,
            KEY_HOMEPAGE => KeyCode::KEY_HOMEPAGE,
            KEY_REFRESH => KeyCode::KEY_REFRESH,
            KEY_EXIT => KeyCode::KEY_EXIT,
            KEY_MOVE => KeyCode::KEY_MOVE,
            KEY_EDIT => KeyCode::KEY_EDIT,
            KEY_SCROLLUP => KeyCode::KEY_SCROLLUP,
            KEY_SCROLLDOWN => KeyCode::KEY_SCROLLDOWN,
            KEY_KPLEFTPAREN => KeyCode::KEY_KPLEFTPAREN,
            KEY_KPRIGHTPAREN => KeyCode::KEY_KPRIGHTPAREN,
            KEY_NEW => KeyCode::KEY_NEW,
            KEY_REDO => KeyCode::KEY_REDO,

            KEY_F13 => KeyCode::KEY_F13,
            KEY_F14 => KeyCode::KEY_F14,
            KEY_F15 => KeyCode::KEY_F15,
            KEY_F16 => KeyCode::KEY_F16,
            KEY_F17 => KeyCode::KEY_F17,
            KEY_F18 => KeyCode::KEY_F18,
            KEY_F19 => KeyCode::KEY_F19,
            KEY_F20 => KeyCode::KEY_F20,
            KEY_F21 => KeyCode::KEY_F21,
            KEY_F22 => KeyCode::KEY_F22,
            KEY_F23 => KeyCode::KEY_F23,
            KEY_F24 => KeyCode::KEY_F24,

            KEY_PLAYCD => KeyCode::KEY_PLAYCD,
            KEY_PAUSECD => KeyCode::KEY_PAUSECD,
            KEY_PROG3 => KeyCode::KEY_PROG3,
            KEY_PROG4 => KeyCode::KEY_PROG4,
            KEY_ALL_APPLICATIONS => KeyCode::KEY_ALL_APPLICATIONS,
            KEY_DASHBOARD => KeyCode::KEY_DASHBOARD,
            KEY_SUSPEND => KeyCode::KEY_SUSPEND,
            KEY_CLOSE => KeyCode::KEY_CLOSE,
            KEY_PLAY => KeyCode::KEY_PLAY,
            KEY_FASTFORWARD => KeyCode::KEY_FASTFORWARD,
            KEY_BASSBOOST => KeyCode::KEY_BASSBOOST,
            KEY_PRINT => KeyCode::KEY_PRINT,
            KEY_HP => KeyCode::KEY_HP,
            KEY_CAMERA => KeyCode::KEY_CAMERA,
            KEY_SOUND => KeyCode::KEY_SOUND,
            KEY_QUESTION => KeyCode::KEY_QUESTION,
            KEY_EMAIL => KeyCode::KEY_EMAIL,
            KEY_CHAT => KeyCode::KEY_CHAT,
            KEY_SEARCH => KeyCode::KEY_SEARCH,
            KEY_CONNECT => KeyCode::KEY_CONNECT,
            KEY_FINANCE => KeyCode::KEY_FINANCE,
            KEY_SPORT => KeyCode::KEY_SPORT,
            KEY_SHOP => KeyCode::KEY_SHOP,
            KEY_ALTERASE => KeyCode::KEY_ALTERASE,
            KEY_CANCEL => KeyCode::KEY_CANCEL,
            KEY_BRIGHTNESSDOWN => KeyCode::KEY_BRIGHTNESSDOWN,
            KEY_BRIGHTNESSUP => KeyCode::KEY_BRIGHTNESSUP,
            KEY_MEDIA => KeyCode::KEY_MEDIA,

            KEY_SWITCHVIDEOMODE => KeyCode::KEY_SWITCHVIDEOMODE,
            KEY_KBDILLUMTOGGLE => KeyCode::KEY_KBDILLUMTOGGLE,
            KEY_KBDILLUMDOWN => KeyCode::KEY_KBDILLUMDOWN,
            KEY_KBDILLUMUP => KeyCode::KEY_KBDILLUMUP,

            KEY_SEND => KeyCode::KEY_SEND,
            KEY_REPLY => KeyCode::KEY_REPLY,
            KEY_FORWARDMAIL => KeyCode::KEY_FORWARDMAIL,
            KEY_SAVE => KeyCode::KEY_SAVE,
            KEY_DOCUMENTS => KeyCode::KEY_DOCUMENTS,

            KEY_BATTERY => KeyCode::KEY_BATTERY,

            KEY_BLUETOOTH => KeyCode::KEY_BLUETOOTH,
            KEY_WLAN => KeyCode::KEY_WLAN,
            KEY_UWB => KeyCode::KEY_UWB,

            KEY_UNKNOWN => KeyCode::KEY_UNKNOWN,

            KEY_VIDEO_NEXT => KeyCode::KEY_VIDEO_NEXT,
            KEY_VIDEO_PREV => KeyCode::KEY_VIDEO_PREV,
            KEY_BRIGHTNESS_CYCLE => KeyCode::KEY_BRIGHTNESS_CYCLE,
            KEY_BRIGHTNESS_AUTO => KeyCode::KEY_BRIGHTNESS_AUTO,

            KEY_BRIGHTNESS_ZERO => KeyCode::KEY_BRIGHTNESS_ZERO,
            KEY_DISPLAY_OFF => KeyCode::KEY_DISPLAY_OFF,

            KEY_WWAN => KeyCode::KEY_WWAN,
            KEY_WIMAX => KeyCode::KEY_WIMAX,
            KEY_RFKILL => KeyCode::KEY_RFKILL,

            KEY_MICMUTE => KeyCode::KEY_MICMUTE,
            _ => KeyCode::KEY_RESERVED,
        }
    }
}

impl Display for KeyCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            KeyCode::KEY_RESERVED => "KEY_RESERVED",
            KeyCode::KEY_ESC => "KEY_ESC",
            KeyCode::KEY_1 => "KEY_1",
            KeyCode::KEY_2 => "KEY_2",
            KeyCode::KEY_3 => "KEY_3",
            KeyCode::KEY_4 => "KEY_4",
            KeyCode::KEY_5 => "KEY_5",
            KeyCode::KEY_6 => "KEY_6",
            KeyCode::KEY_7 => "KEY_7",
            KeyCode::KEY_8 => "KEY_8",
            KeyCode::KEY_9 => "KEY_9",
            KeyCode::KEY_0 => "KEY_0",
            KeyCode::KEY_MINUS => "KEY_MINUS",
            KeyCode::KEY_EQUAL => "KEY_EQUAL",
            KeyCode::KEY_BACKSPACE => "KEY_BACKSPACE",
            KeyCode::KEY_TAB => "KEY_TAB",
            KeyCode::KEY_Q => "KEY_Q",
            KeyCode::KEY_W => "KEY_W",
            KeyCode::KEY_E => "KEY_E",
            KeyCode::KEY_R => "KEY_R",
            KeyCode::KEY_T => "KEY_T",
            KeyCode::KEY_Y => "KEY_Y",
            KeyCode::KEY_U => "KEY_U",
            KeyCode::KEY_I => "KEY_I",
            KeyCode::KEY_O => "KEY_O",
            KeyCode::KEY_P => "KEY_P",
            KeyCode::KEY_LEFTBRACE => "KEY_LEFTBRACE",
            KeyCode::KEY_RIGHTBRACE => "KEY_RIGHTBRACE",
            KeyCode::KEY_ENTER => "KEY_ENTER",
            KeyCode::KEY_LEFTCTRL => "KEY_LEFTCTRL",
            KeyCode::KEY_A => "KEY_A",
            KeyCode::KEY_S => "KEY_S",
            KeyCode::KEY_D => "KEY_D",
            KeyCode::KEY_F => "KEY_F",
            KeyCode::KEY_G => "KEY_G",
            KeyCode::KEY_H => "KEY_H",
            KeyCode::KEY_J => "KEY_J",
            KeyCode::KEY_K => "KEY_K",
            KeyCode::KEY_L => "KEY_L",
            KeyCode::KEY_SEMICOLON => "KEY_SEMICOLON",
            KeyCode::KEY_APOSTROPHE => "KEY_APOSTROPHE",
            KeyCode::KEY_GRAVE => "KEY_GRAVE",
            KeyCode::KEY_LEFTSHIFT => "KEY_LEFTSHIFT",
            KeyCode::KEY_BACKSLASH => "KEY_BACKSLASH",
            KeyCode::KEY_Z => "KEY_Z",
            KeyCode::KEY_X => "KEY_X",
            KeyCode::KEY_C => "KEY_C",
            KeyCode::KEY_V => "KEY_V",
            KeyCode::KEY_B => "KEY_B",
            KeyCode::KEY_N => "KEY_N",
            KeyCode::KEY_M => "KEY_M",
            KeyCode::KEY_COMMA => "KEY_COMMA",
            KeyCode::KEY_DOT => "KEY_DOT",
            KeyCode::KEY_SLASH => "KEY_SLASH",
            KeyCode::KEY_RIGHTSHIFT => "KEY_RIGHTSHIFT",
            KeyCode::KEY_KPASTERISK => "KEY_KPASTERISK",
            KeyCode::KEY_LEFTALT => "KEY_LEFTALT",
            KeyCode::KEY_SPACE => "KEY_SPACE",
            KeyCode::KEY_CAPSLOCK => "KEY_CAPSLOCK",
            KeyCode::KEY_F1 => "KEY_F1",
            KeyCode::KEY_F2 => "KEY_F2",
            KeyCode::KEY_F3 => "KEY_F3",
            KeyCode::KEY_F4 => "KEY_F4",
            KeyCode::KEY_F5 => "KEY_F5",
            KeyCode::KEY_F6 => "KEY_F6",
            KeyCode::KEY_F7 => "KEY_F7",
            KeyCode::KEY_F8 => "KEY_F8",
            KeyCode::KEY_F9 => "KEY_F9",
            KeyCode::KEY_F10 => "KEY_F10",
            KeyCode::KEY_NUMLOCK => "KEY_NUMLOCK",
            KeyCode::KEY_SCROLLLOCK => "KEY_SCROLLLOCK",
            KeyCode::KEY_KP7 => "KEY_KP7",
            KeyCode::KEY_KP8 => "KEY_KP8",
            KeyCode::KEY_KP9 => "KEY_KP9",
            KeyCode::KEY_KPMINUS => "KEY_KPMINUS",
            KeyCode::KEY_KP4 => "KEY_KP4",
            KeyCode::KEY_KP5 => "KEY_KP5",
            KeyCode::KEY_KP6 => "KEY_KP6",
            KeyCode::KEY_KPPLUS => "KEY_KPPLUS",
            KeyCode::KEY_KP1 => "KEY_KP1",
            KeyCode::KEY_KP2 => "KEY_KP2",
            KeyCode::KEY_KP3 => "KEY_KP3",
            KeyCode::KEY_KP0 => "KEY_KP0",
            KeyCode::KEY_KPDOT => "KEY_KPDOT",

            KeyCode::KEY_ZENKAKUHANKAKU => "KEY_ZENKAKUHANKAKU",
            KeyCode::KEY_102ND => "KEY_102ND",
            KeyCode::KEY_F11 => "KEY_F11",
            KeyCode::KEY_F12 => "KEY_F12",
            KeyCode::KEY_RO => "KEY_RO",
            KeyCode::KEY_KATAKANA => "KEY_KATAKANA",
            KeyCode::KEY_HIRAGANA => "KEY_HIRAGANA",
            KeyCode::KEY_HENKAN => "KEY_HENKAN",
            KeyCode::KEY_KATAKANAHIRAGANA => "KEY_KATAKANAHIRAGANA",
            KeyCode::KEY_MUHENKAN => "KEY_MUHENKAN",
            KeyCode::KEY_KPJPCOMMA => "KEY_KPJPCOMMA",
            KeyCode::KEY_KPENTER => "KEY_KPENTER",
            KeyCode::KEY_RIGHTCTRL => "KEY_RIGHTCTRL",
            KeyCode::KEY_KPSLASH => "KEY_KPSLASH",
            KeyCode::KEY_SYSRQ => "KEY_SYSRQ",
            KeyCode::KEY_RIGHTALT => "KEY_RIGHTALT",
            KeyCode::KEY_LINEFEED => "KEY_LINEFEED",
            KeyCode::KEY_HOME => "KEY_HOME",
            KeyCode::KEY_UP => "KEY_UP",
            KeyCode::KEY_PAGEUP => "KEY_PAGEUP",
            KeyCode::KEY_LEFT => "KEY_LEFT",
            KeyCode::KEY_RIGHT => "KEY_RIGHT",
            KeyCode::KEY_END => "KEY_END",
            KeyCode::KEY_DOWN => "KEY_DOWN",
            KeyCode::KEY_PAGEDOWN => "KEY_PAGEDOWN",
            KeyCode::KEY_INSERT => "KEY_INSERT",
            KeyCode::KEY_DELETE => "KEY_DELETE",
            KeyCode::KEY_MACRO => "KEY_MACRO",
            KeyCode::KEY_MUTE => "KEY_MUTE",
            KeyCode::KEY_VOLUMEDOWN => "KEY_VOLUMEDOWN",
            KeyCode::KEY_VOLUMEUP => "KEY_VOLUMEUP",
            KeyCode::KEY_POWER => "KEY_POWER",
            KeyCode::KEY_KPEQUAL => "KEY_KPEQUAL",
            KeyCode::KEY_KPPLUSMINUS => "KEY_KPPLUSMINUS",
            KeyCode::KEY_PAUSE => "KEY_PAUSE",
            KeyCode::KEY_SCALE => "KEY_SCALE",

            KeyCode::KEY_KPCOMMA => "KEY_KPCOMMA",
            KeyCode::KEY_HANGEUL => "KEY_HANGEUL",
            KeyCode::KEY_HANGUEL => "KEY_HANGUEL",
            KeyCode::KEY_HANJA => "KEY_HANJA",
            KeyCode::KEY_YEN => "KEY_YEN",
            KeyCode::KEY_LEFTMETA => "KEY_LEFTMETA",
            KeyCode::KEY_RIGHTMETA => "KEY_RIGHTMETA",
            KeyCode::KEY_COMPOSE => "KEY_COMPOSE",

            KeyCode::KEY_STOP => "KEY_STOP",
            KeyCode::KEY_AGAIN => "KEY_AGAIN",
            KeyCode::KEY_PROPS => "KEY_PROPS",
            KeyCode::KEY_UNDO => "KEY_UNDO",
            KeyCode::KEY_FRONT => "KEY_FRONT",
            KeyCode::KEY_COPY => "KEY_COPY",
            KeyCode::KEY_OPEN => "KEY_OPEN",
            KeyCode::KEY_PASTE => "KEY_PASTE",
            KeyCode::KEY_FIND => "KEY_FIND",
            KeyCode::KEY_CUT => "KEY_CUT",
            KeyCode::KEY_HELP => "KEY_HELP",
            KeyCode::KEY_MENU => "KEY_MENU",
            KeyCode::KEY_CALC => "KEY_CALC",
            KeyCode::KEY_SETUP => "KEY_SETUP",
            KeyCode::KEY_SLEEP => "KEY_SLEEP",
            KeyCode::KEY_WAKEUP => "KEY_WAKEUP",
            KeyCode::KEY_FILE => "KEY_FILE",
            KeyCode::KEY_SENDFILE => "KEY_SENDFILE",
            KeyCode::KEY_DELETEFILE => "KEY_DELETEFILE",
            KeyCode::KEY_XFER => "KEY_XFER",
            KeyCode::KEY_PROG1 => "KEY_PROG1",
            KeyCode::KEY_PROG2 => "KEY_PROG2",
            KeyCode::KEY_WWW => "KEY_WWW",
            KeyCode::KEY_MSDOS => "KEY_MSDOS",
            KeyCode::KEY_COFFEE => "KEY_COFFEE",
            KeyCode::KEY_SCREENLOCK => "KEY_SCREENLOCK",
            KeyCode::KEY_ROTATE_DISPLAY => "KEY_ROTATE_DISPLAY",
            KeyCode::KEY_DIRECTION => "KEY_DIRECTION",
            KeyCode::KEY_CYCLEWINDOWS => "KEY_CYCLEWINDOWS",
            KeyCode::KEY_MAIL => "KEY_MAIL",
            KeyCode::KEY_BOOKMARKS => "KEY_BOOKMARKS",
            KeyCode::KEY_COMPUTER => "KEY_COMPUTER",
            KeyCode::KEY_BACK => "KEY_BACK",
            KeyCode::KEY_FORWARD => "KEY_FORWARD",
            KeyCode::KEY_CLOSECD => "KEY_CLOSECD",
            KeyCode::KEY_EJECTCD => "KEY_EJECTCD",
            KeyCode::KEY_EJECTCLOSECD => "KEY_EJECTCLOSECD",
            KeyCode::KEY_NEXTSONG => "KEY_NEXTSONG",
            KeyCode::KEY_PLAYPAUSE => "KEY_PLAYPAUSE",
            KeyCode::KEY_PREVIOUSSONG => "KEY_PREVIOUSSONG",
            KeyCode::KEY_STOPCD => "KEY_STOPCD",
            KeyCode::KEY_RECORD => "KEY_RECORD",
            KeyCode::KEY_REWIND => "KEY_REWIND",
            KeyCode::KEY_PHONE => "KEY_PHONE",
            KeyCode::KEY_ISO => "KEY_ISO",
            KeyCode::KEY_CONFIG => "KEY_CONFIG",
            KeyCode::KEY_HOMEPAGE => "KEY_HOMEPAGE",
            KeyCode::KEY_REFRESH => "KEY_REFRESH",
            KeyCode::KEY_EXIT => "KEY_EXIT",
            KeyCode::KEY_MOVE => "KEY_MOVE",
            KeyCode::KEY_EDIT => "KEY_EDIT",
            KeyCode::KEY_SCROLLUP => "KEY_SCROLLUP",
            KeyCode::KEY_SCROLLDOWN => "KEY_SCROLLDOWN",
            KeyCode::KEY_KPLEFTPAREN => "KEY_KPLEFTPAREN",
            KeyCode::KEY_KPRIGHTPAREN => "KEY_KPRIGHTPAREN",
            KeyCode::KEY_NEW => "KEY_NEW",
            KeyCode::KEY_REDO => "KEY_REDO",

            KeyCode::KEY_F13 => "KEY_F13",
            KeyCode::KEY_F14 => "KEY_F14",
            KeyCode::KEY_F15 => "KEY_F15",
            KeyCode::KEY_F16 => "KEY_F16",
            KeyCode::KEY_F17 => "KEY_F17",
            KeyCode::KEY_F18 => "KEY_F18",
            KeyCode::KEY_F19 => "KEY_F19",
            KeyCode::KEY_F20 => "KEY_F20",
            KeyCode::KEY_F21 => "KEY_F21",
            KeyCode::KEY_F22 => "KEY_F22",
            KeyCode::KEY_F23 => "KEY_F23",
            KeyCode::KEY_F24 => "KEY_F24",

            KeyCode::KEY_PLAYCD => "KEY_PLAYCD",
            KeyCode::KEY_PAUSECD => "KEY_PAUSECD",
            KeyCode::KEY_PROG3 => "KEY_PROG3",
            KeyCode::KEY_PROG4 => "KEY_PROG4",
            KeyCode::KEY_ALL_APPLICATIONS => "KEY_ALL_APPLICATIONS",
            KeyCode::KEY_DASHBOARD => "KEY_DASHBOARD",
            KeyCode::KEY_SUSPEND => "KEY_SUSPEND",
            KeyCode::KEY_CLOSE => "KEY_CLOSE",
            KeyCode::KEY_PLAY => "KEY_PLAY",
            KeyCode::KEY_FASTFORWARD => "KEY_FASTFORWARD",
            KeyCode::KEY_BASSBOOST => "KEY_BASSBOOST",
            KeyCode::KEY_PRINT => "KEY_PRINT",
            KeyCode::KEY_HP => "KEY_HP",
            KeyCode::KEY_CAMERA => "KEY_CAMERA",
            KeyCode::KEY_SOUND => "KEY_SOUND",
            KeyCode::KEY_QUESTION => "KEY_QUESTION",
            KeyCode::KEY_EMAIL => "KEY_EMAIL",
            KeyCode::KEY_CHAT => "KEY_CHAT",
            KeyCode::KEY_SEARCH => "KEY_SEARCH",
            KeyCode::KEY_CONNECT => "KEY_CONNECT",
            KeyCode::KEY_FINANCE => "KEY_FINANCE",
            KeyCode::KEY_SPORT => "KEY_SPORT",
            KeyCode::KEY_SHOP => "KEY_SHOP",
            KeyCode::KEY_ALTERASE => "KEY_ALTERASE",
            KeyCode::KEY_CANCEL => "KEY_CANCEL",
            KeyCode::KEY_BRIGHTNESSDOWN => "KEY_BRIGHTNESSDOWN",
            KeyCode::KEY_BRIGHTNESSUP => "KEY_BRIGHTNESSUP",
            KeyCode::KEY_MEDIA => "KEY_MEDIA",

            KeyCode::KEY_SWITCHVIDEOMODE => "KEY_SWITCHVIDEOMODE",
            KeyCode::KEY_KBDILLUMTOGGLE => "KEY_KBDILLUMTOGGLE",
            KeyCode::KEY_KBDILLUMDOWN => "KEY_KBDILLUMDOWN",
            KeyCode::KEY_KBDILLUMUP => "KEY_KBDILLUMUP",

            KeyCode::KEY_SEND => "KEY_SEND",
            KeyCode::KEY_REPLY => "KEY_REPLY",
            KeyCode::KEY_FORWARDMAIL => "KEY_FORWARDMAIL",
            KeyCode::KEY_SAVE => "KEY_SAVE",
            KeyCode::KEY_DOCUMENTS => "KEY_DOCUMENTS",

            KeyCode::KEY_BATTERY => "KEY_BATTERY",

            KeyCode::KEY_BLUETOOTH => "KEY_BLUETOOTH",
            KeyCode::KEY_WLAN => "KEY_WLAN",
            KeyCode::KEY_UWB => "KEY_UWB",

            KeyCode::KEY_UNKNOWN => "KEY_UNKNOWN",

            KeyCode::KEY_VIDEO_NEXT => "KEY_VIDEO_NEXT",
            KeyCode::KEY_VIDEO_PREV => "KEY_VIDEO_PREV",
            KeyCode::KEY_BRIGHTNESS_CYCLE => "KEY_BRIGHTNESS_CYCLE",
            KeyCode::KEY_BRIGHTNESS_AUTO => "KEY_BRIGHTNESS_AUTO",

            KeyCode::KEY_BRIGHTNESS_ZERO => "KEY_BRIGHTNESS_ZERO",
            KeyCode::KEY_DISPLAY_OFF => "KEY_DISPLAY_OFF",

            KeyCode::KEY_WWAN => "KEY_WWAN",
            KeyCode::KEY_WIMAX => "KEY_WIMAX",
            KeyCode::KEY_RFKILL => "KEY_RFKILL",

            KeyCode::KEY_MICMUTE => "KEY_MICMUTE",
        };
        f.write_str(str)
    }
}
