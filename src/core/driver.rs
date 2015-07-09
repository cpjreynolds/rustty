use util::errors::{
    Result,
    Error,
};

use term::terminfo::TermInfo;
use term::terminfo::parm;
use term::terminfo::parm::{
    Param,
    Variables,
};

// String constants correspond to terminfo capnames and are used internally for name resolution.
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

// Array of required capabilities, used as an iterator on startup to ensure all required
// functionality is present.
static CAP_TABLE: &'static [&'static str] = &[
    ENTER_CA,
    EXIT_CA,
    SHOW_CURSOR,
    HIDE_CURSOR,
    SET_CURSOR,
    CLEAR,
    RESET,
    UNDERLINE,
    BOLD,
    REVERSE,
    SETFG,
    SETBG,
];

// Driver capabilities are an enum instead of string constants (there are string constants private
// to the module however, those are only used for naming convenience and disambiguation)
// to take advantage of compile-time type-checking instead of hoping capability names are correct.
// This allows us to guarantee that driver accesses will succeed. In addition, using an enum means
// Driver doesn't need hard-coded methods for each capability we want to use.
pub enum Cap {
    EnterCa,
    ExitCa,
    ShowCursor,
    HideCursor,
    SetCursor(i16, i16),
    Clear,
    Reset,
    Underline,
    Bold,
    Blink,
    Reverse,
    SetFg(u8),
    SetBg(u8),
}

impl Cap {
    fn resolve(&self) -> &'static str {
        match *self {
            Cap::EnterCa => ENTER_CA,
            Cap::ExitCa => EXIT_CA,
            Cap::ShowCursor => SHOW_CURSOR,
            Cap::HideCursor => HIDE_CURSOR,
            Cap::SetCursor(..) => SET_CURSOR,
            Cap::Clear => CLEAR,
            Cap::Reset => RESET,
            Cap::Underline => UNDERLINE,
            Cap::Bold => BOLD,
            Cap::Blink => BLINK,
            Cap::Reverse => REVERSE,
            Cap::SetFg(..) => SETFG,
            Cap::SetBg(..) => SETBG,
        }
    }
}

pub struct Driver {
    tinfo: TermInfo,
}

impl Driver {
    pub fn new() -> Result<Driver> {
        let tinfo = try!(TermInfo::from_env());
        for capname in CAP_TABLE {
            if !tinfo.strings.contains_key(*capname) {
                return Err(Error::new(&format!("terminal missing capability: '{}'", capname)));
            }
        }
        Ok(Driver {
            tinfo: tinfo,
        })
    }

    // get() will not return an error, and theoretically should never panic.
    //
    // The pre-flight checks on initialization of `Driver` ensure that every capability is present,
    // thus the `HashMap` retrieval should never fail.
    // Furthermore the `expand()` routine, given the input we pass it, should never fail either.
    // This can be verified in the source of the `term` crate.
    pub fn get(&self, cap_request: Cap) -> Vec<u8> {
        let capname = cap_request.resolve();
        let cap = self.tinfo.strings.get(capname).unwrap();

        match cap_request {
            Cap::SetFg(attr) | Cap::SetBg(attr) => {
                let params = &[Param::Number(attr as i16)];
                let mut vars = Variables::new();
                parm::expand(cap, params, &mut vars).unwrap()
            },
            Cap::SetCursor(x, y) => {
                let params = &[Param::Number(x), Param::Number(y)];
                let mut vars = Variables::new();
                parm::expand(cap, params, &mut vars).unwrap()
            },
            _ => {
                cap.clone()
            },
        }
    }
}

