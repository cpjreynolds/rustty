// Temporary fix before certain constants are used.
#![allow(dead_code)]

use term::Error;
use term::terminfo::TermInfo;
use term::terminfo::parm;
use term::terminfo::parm::{Param, Variables};

// Terminfo keys. These are arrays because the terminfo database from the `term` crate sometimes
// uses the variable name and othertimes the capname.
//
// Arrays are formatted as ["variable_name", "cap_name"].
const KEY_F1: &'static [&'static str] = &["key_f1", "kf1"];
const KEY_F2: &'static [&'static str] = &["key_f2", "kf2"];
const KEY_F3: &'static [&'static str] = &["key_f3", "kf3"];
const KEY_F4: &'static [&'static str] = &["key_f4", "kf4"];
const KEY_F5: &'static [&'static str] = &["key_f5", "kf5"];
const KEY_F6: &'static [&'static str] = &["key_f6", "kf6"];
const KEY_F7: &'static [&'static str] = &["key_f7", "kf7"];
const KEY_F8: &'static [&'static str] = &["key_f8", "kf8"];
const KEY_F9: &'static [&'static str] = &["key_f9", "kf9"];
const KEY_F10: &'static [&'static str] = &["key_f10", "kf10"];
const KEY_F11: &'static [&'static str] = &["key_f11", "kf11"];
const KEY_F12: &'static [&'static str] = &["key_f12", "kf12"];
const KEY_UP: &'static [&'static str] = &["key_up", "kcuu1"];
const KEY_DOWN: &'static [&'static str] = &["key_down", "kcud1"];
const KEY_LEFT: &'static [&'static str] = &["key_left", "kcub1"];
const KEY_RIGHT: &'static [&'static str] = &["key_right", "kcuf1"];

// Array of terminal keys.
const KEYS: &'static [&'static [&'static str]] = &[KEY_F1, KEY_F2, KEY_F3, KEY_F4, KEY_F5, KEY_F6,
                                                   KEY_F7, KEY_F8, KEY_F9, KEY_F10, KEY_F11,
                                                   KEY_F12, KEY_UP, KEY_DOWN, KEY_LEFT, KEY_RIGHT];

// String constants correspond to terminfo capnames and are used inside the module for convenience.
const ENTER_CA: &'static str = "smcup";
const EXIT_CA: &'static str = "rmcup";
const SHOW_CURSOR: &'static str = "cnorm";
const HIDE_CURSOR: &'static str = "civis";
const SET_CURSOR: &'static str = "cup";
const CLEAR: &'static str = "clear";
const RESET: &'static str = "sgr0";
const UNDERLINE: &'static str = "smul";
const BOLD: &'static str = "bold";
const BLINK: &'static str = "blink";
const REVERSE: &'static str = "rev";
const SETFG: &'static str = "setaf";
const SETBG: &'static str = "setab";

// Driver capabilities are an enum instead of string constants (there are string constants private
// to the module however, those are only used for naming convenience and disambiguation)
// to take advantage of compile-time type-checking instead of hoping invalid strings aren't passed.
// In addition, using an enum means Driver doesn't need hard-coded methods for each capability we
// want to use.
pub enum DevFn {
    EnterCa,
    ExitCa,
    ShowCursor,
    HideCursor,
    SetCursor(usize, usize),
    Clear,
    Reset,
    Underline,
    Bold,
    Blink,
    Reverse,
    SetFg(u8),
    SetBg(u8),
}

impl DevFn {
    fn as_str(&self) -> &'static str {
        match *self {
            DevFn::EnterCa => ENTER_CA,
            DevFn::ExitCa => EXIT_CA,
            DevFn::ShowCursor => SHOW_CURSOR,
            DevFn::HideCursor => HIDE_CURSOR,
            DevFn::SetCursor(..) => SET_CURSOR,
            DevFn::Clear => CLEAR,
            DevFn::Reset => RESET,
            DevFn::Underline => UNDERLINE,
            DevFn::Bold => BOLD,
            DevFn::Blink => BLINK,
            DevFn::Reverse => REVERSE,
            DevFn::SetFg(..) => SETFG,
            DevFn::SetBg(..) => SETBG,
        }
    }
}

pub struct Driver {
    tinfo: TermInfo,
}

impl Driver {
    // Creates a new `Driver`
    pub fn new() -> Result<Driver, Error> {
        let tinfo = try!(TermInfo::from_env());
        Ok(Driver { tinfo: tinfo })
    }

    // Returns the device specific escape sequence for the given `DevFn`, or None if the terminal
    // lacks the capability to perform the specified function.
    pub fn get(&self, dfn: DevFn) -> Option<Vec<u8>> {
        let capname = dfn.as_str();
        self.tinfo.strings.get(capname).map(|cap| {

            match dfn {
                DevFn::SetFg(attr) |
                DevFn::SetBg(attr) => {
                    let params = &[Param::Number(attr as i32)];
                    let mut vars = Variables::new();
                    parm::expand(cap, params, &mut vars).unwrap()
                }
                DevFn::SetCursor(x, y) => {
                    let params = &[Param::Number(y as i32), Param::Number(x as i32)];
                    let mut vars = Variables::new();
                    parm::expand(cap, params, &mut vars).unwrap()
                }
                _ => cap.clone(),
            }
        })
    }
}
