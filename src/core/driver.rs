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

// Array of terminal capabilities. Used as an iterator to test for functionality.
//
// At the moment all functionality is required, however in the future we should implement optional
// functionality checks so the absence of underlining or reverse video doesn't cause initialization
// to fail.
static CAPABILITIES: &'static [&'static str] = &[
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
// to take advantage of compile-time type-checking instead of hoping invalid strings aren't passed.
// This allows us to guarantee that driver accesses will succeed. In addition, using an enum means
// Driver doesn't need hard-coded methods for each capability we want to use.
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
    tinfo: &'static TermInfo,
}

lazy_static! {
    static ref TINFO: TermInfo = {
        TermInfo::from_env().unwrap_or({
            TermInfo {
                names: Default::default(),
                bools: Default::default(),
                numbers: Default::default(),
                strings: Default::default(),
            }
        })
    };
}

fn get_tinfo() -> Result<&'static TermInfo> {
    if let Err(e) = validate_tinfo(&*TINFO) {
        Err(e)
    } else {
        Ok(&*TINFO)
    }
}

fn validate_tinfo(tinfo: &TermInfo) -> Result<()> {
    for capname in CAPABILITIES {
        if !tinfo.strings.contains_key(*capname) {
            return Err(Error::new(&format!("terminal missing capability: '{}'", capname)));
        }
    }
    Ok(())
}

impl Driver {
    pub fn new() -> Result<Driver> {
        let tinfo = try!(get_tinfo());
        Ok(Driver {
            tinfo: tinfo,
        })
    }

    // Processes a capability and returns the device specific escape sequence.
    //
    // process() will not return an error, and theoretically should never panic.
    //
    // The pre-flight checks on initialization of `Driver` ensure that every capability is present,
    // thus the `HashMap` retrieval should never fail.
    // Furthermore the `expand()` routine, given the input we pass it, should never fail either.
    // This can be verified in the source of the `term` crate.
    pub fn process(&self, dfn: DevFn) -> Vec<u8> {
        let capname = dfn.as_str();
        let cap = self.tinfo.strings.get(capname).unwrap();

        match dfn {
            DevFn::SetFg(attr) | DevFn::SetBg(attr) => {
                let params = &[Param::Number(attr as i16)];
                let mut vars = Variables::new();
                parm::expand(cap, params, &mut vars).unwrap()
            },
            DevFn::SetCursor(x, y) => {
                let params = &[Param::Number(y as i16), Param::Number(x as i16)];
                let mut vars = Variables::new();
                parm::expand(cap, params, &mut vars).unwrap()
            },
            _ => {
                cap.clone()
            },
        }
    }
}

