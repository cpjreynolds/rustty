#[macro_use]
extern crate bitflags;
extern crate libc;
extern crate nix;

static XTERM_FUNCS: &'static [&'static str] = &[
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

static XTERM_KEYS: &'static [&'static str] = &[
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

