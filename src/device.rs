use std::env;
use std::ops::Index;

use TtyError;

pub struct Device {
    name: &'static str,
    keys: &'static [&'static str],
    funcs: &'static [&'static str],
}

impl Device {
    pub fn new() -> Result<&'static Device, TtyError> {
        if let Ok(dname) = env::var("TERM") {
            if let Some(dev) = DEVICES.iter().find(|d| { dname.contains(d.name) }) {
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

impl Index<DevFunc> for Device {
    type Output = [u8];

    fn index(&self, index: DevFunc) -> &[u8] {
        self.funcs[index as usize].as_bytes()
    }
}

pub enum DevFunc {
    EnterCa,
    ExitCa,
    ShowCursor,
    HideCursor,
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
        name: "xterm",
        funcs: &[
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
        ],
        keys: &[
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
        ],
    },
];

