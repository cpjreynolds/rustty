use std::env;
use std::ops::Index;

use TtyError;

const XTERM_FUNCS: &'static [&'static str] = &[
    "\x1b[?1049h",
    "\x1b[?1049l",
    "\x1b[?12l\x1b[?25h",
    "\x1b[?25l",
    "\x1b[H\x1b[2J",
    "\x1b(B\x1b[m",
    "\x1b[4m",
    "\x1b[1m",
    "\x1b[5m",
    "\x1b[7m",
    "\x1b[?1h\x1b=",
    "\x1b[?1l\x1b>",
    "\x1b[?1000h",
    "\x1b[?1000l",
];

const XTERM_KEYS: &'static [&'static str] = &[
    "\x1bOP",
    "\x1bOQ",
    "\x1bOR",
    "\x1bOS",
    "\x1b[15~",
    "\x1b[17~",
    "\x1b[18~",
    "\x1b[19~",
    "\x1b[20~",
    "\x1b[21~",
    "\x1b[23~",
    "\x1b[24~",
    "\x1b[2~",
    "\x1b[3~",
    "\x1bOH",
    "\x1bOF",
    "\x1b[5~",
    "\x1b[6~",
    "\x1bOA",
    "\x1bOB",
    "\x1bOD",
    "\x1bOC",
];

pub struct Device {
    name: &'static str,
    keys: &'static [&'static str],
    funcs: &'static [&'static str],
}

impl Device {
    pub fn new() -> Result<&'static Device, TtyError> {
        if let Ok(dname) = env::var("TERM") {
            if let Some(dev) = DEVICES.iter().find(|d| { d.name == dname }) {
                Ok(dev)
            } else {
                Err(TtyError::new("Unsupported terminal"))
            }
        } else {
            Err(TtyError::new("TERM not set"))
        }
    }

    pub fn name(&self) -> &str {
        self.name
    }
}

impl Index<DFunction> for Device {
    type Output = str;

    fn index(&self, index: DFunction) -> &str {
        self.funcs[index as usize]
    }
}

pub enum DFunction {
    EnterCa,
    ExitCa,
    ShowCursor,
    HideScreen,
    ClearScreen,
    Sgr0,
    Underline,
    Bold,
    Blink,
    Reverse,
    EnterKeypad,
    ExitKeypad,
    EnterMouse,
    ExitMouse,
    FuncsNum,
}

const DEVICES: &'static [Device] = &[
    Device {
        name: "xterm-256color",
        keys: XTERM_KEYS,
        funcs: XTERM_FUNCS,
    },
];

