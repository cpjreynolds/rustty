use std::io::prelude::*;
use std::io::{Error, ErrorKind, Result};
use std::os::unix::io::AsRawFd;
use std::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};
use std::collections::{VecDeque, vec_deque};
use std::thread;
use std::time::{Duration, Instant};
use std::ptr;
use std::mem;
use std::iter::Iterator;

use libc;

use gag::BufferRedirect;

use core::cell::{Cell, Color, BOLD, UNDERLINE, REVERSE};
use core::panel::{Panel, Draw};
use core::input::Event;
use core::driver::{DevFn, Driver};
use core::tty::{self, RawTerminal, ControlChar};

// Set to true by the sigwinch handler. Reset to false when buffers are resized.
static SIGWINCH_STATUS: AtomicBool = ATOMIC_BOOL_INIT;

// Ensures that there is only ever one Terminal object at any one time.
// Set to true on creation of a Terminal object.
// Reset to false when terminal object goes out of scope.
static RUSTTY_STATUS: AtomicBool = ATOMIC_BOOL_INIT;

type OutBuffer = Vec<u8>;
type EventBuffer = VecDeque<Event>;

/// A representation of the current terminal window.
///
/// Only one `Terminal` object can exist at any one time, `Terminal::new()` will return an `Error`
/// if a `Terminal` object already exists.
/// When a `Terminal` goes out of scope it resets the underlying terminal to its original state.
///
/// # Examples
///
/// ```no_run
/// # use std::io::{Error, Result};
/// use rustty::{Terminal, Cell, Color};
///
/// # fn foo() -> Result<()> {
///
/// // Construct a new Terminal.
/// let mut term = try!(Terminal::new());
///
/// // Terminals can be indexed to access specific cells.
/// // Indices are by column then row, corresponding to a cell's x and y coordinates.
/// term[(0, 0)].set_ch('x');
/// assert_eq!(term[(0, 0)].ch(), 'x');
///
/// term[(0, 1)].set_bg(Color::Red);
/// assert_eq!(term[(0, 1)].bg(), Color::Red);
///
/// term[(0, 2)].set_fg(Color::Blue);
/// assert_eq!(term[(0, 2)].fg(), Color::Blue);
///
/// # Ok(())
/// # }
/// ```
pub struct Terminal {
    // Raw terminal interface.
    tty: RawTerminal,
    // Terminal driver (terminfo).
    driver: Driver,
    // Internal backbuffer.
    backbuffer: Panel,
    // Internal frontbuffer.
    frontbuffer: Panel,
    // Internal output buffer.
    outbuffer: OutBuffer,
    // Event buffer.
    eventbuffer: EventBuffer,
    // Stderr handle to dump on panics.
    stderr_handle: BufferRedirect,
}

impl Terminal {
    /// Constructs a new `Terminal`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::io::{Error, Result};
    /// use rustty::Terminal;
    ///
    /// # fn foo() -> Result<()> {
    ///
    /// let mut term = try!(Terminal::new());
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn new() -> Result<Terminal> {
        // Make sure there is only ever one instance.
        if RUSTTY_STATUS.compare_and_swap(false, true, Ordering::SeqCst) {
            return Err(Error::new(ErrorKind::AlreadyExists, "terminal already initialized"));
        }

        // Create the terminal driver.
        // If this returns an error then the terminal is not supported.
        let driver = try!(Driver::new());

        // Open terminal interface.
        let tty = try!(RawTerminal::new());

        // Set up the signal handler for SIGWINCH, which will notify us when the window size has
        // changed; it does this by atomically setting SIGWINCH_STATUS to 'true'.
        let handler = sigwinch_handler as libc::size_t;
        let mut sa_winch: libc::sigaction = unsafe { mem::zeroed() };
        sa_winch.sa_sigaction = handler;
        let res = unsafe { libc::sigaction(libc::SIGWINCH, &sa_winch, ptr::null_mut()) };
        if res != 0 {
            return Err(Error::last_os_error());
        }

        let mut terminal = Terminal {
            tty: tty,
            driver: driver,
            backbuffer: Panel::new(),
            frontbuffer: Panel::new(),
            outbuffer: OutBuffer::with_capacity(32 * 1024),
            eventbuffer: EventBuffer::with_capacity(128),
            stderr_handle: BufferRedirect::stderr().unwrap(),
        };

        // set `termios` options.
        let mut tios = terminal.tty.termios();
        tios.iflags_mut()
            .remove(tty::IGNBRK | tty::BRKINT | tty::PARMRK | tty::ISTRIP | tty::INLCR |
                    tty::IGNCR | tty::ICRNL | tty::IXON);
        tios.oflags_mut().remove(tty::OPOST);
        tios.lflags_mut()
            .remove(tty::ECHO | tty::ECHONL | tty::ICANON | tty::ISIG | tty::IEXTEN);
        tios.cflags_mut().remove(tty::CSIZE | tty::PARENB);
        tios.cflags_mut().insert(tty::CS8);
        tios.set_cc(ControlChar::VMIN, 0);
        tios.set_cc(ControlChar::VTIME, 0);
        try!(terminal.tty.set_termios(tios));

        // Switch to alternate screen buffer. Writes the control code to the output buffer.
        try!(terminal.outbuffer.write_all(&terminal.driver.get(DevFn::EnterCa)));

        // Hide cursor. Writes the control code to the output buffer.
        try!(terminal.outbuffer.write_all(&terminal.driver.get(DevFn::HideCursor)));

        // Resize the buffers to the size of the underlying terminal.
        try!(terminal.resize());

        // Return the initialized `Terminal`.
        Ok(terminal)
    }

    /// Updates the underlying terminal, displaying the current backbuffer.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::io::{Error, Result};
    /// use rustty::Terminal;
    ///
    /// # fn foo() -> Result<()> {
    ///
    /// let mut term = try!(Terminal::new());
    /// try!(term.refresh());
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn refresh(&mut self) -> Result<()> {
        // Check whether the window has been resized; if it has then update and resize the buffers.
        if SIGWINCH_STATUS.compare_and_swap(true, false, Ordering::SeqCst) {
            try!(self.resize());
        }

        for y in 0..self.rows() {
            for x in 0..self.cols() {
                if self.frontbuffer[(x, y)] == self.backbuffer[(x, y)] {
                    continue; // Don't redraw cells that haven't changed.
                } else {
                    let cell = self.backbuffer[(x, y)];
                    try!(self.send_style(cell));
                    try!(self.send_char(x, y, cell.ch()));
                    // Update the frontbuffer to reflect the changes.
                    self.frontbuffer[(x, y)] = cell;
                }
            }
        }
        try!(self.flush());
        Ok(())
    }

    /// Returns the width of the terminal in columns.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::io::{Error, Result};
    /// use rustty::Terminal;
    ///
    /// # fn foo() -> Result<()> {
    ///
    /// let mut term = try!(Terminal::new());
    /// let width = term.cols();
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn cols(&self) -> usize {
        self.backbuffer.cols()
    }

    /// Returns the height of the terminal in rows.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::io::{Error, Result};
    /// use rustty::Terminal;
    ///
    /// # fn foo() -> Result<()> {
    ///
    /// let mut term = try!(Terminal::new());
    /// let height = term.rows();
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn rows(&self) -> usize {
        self.backbuffer.rows()
    }

    pub fn panel(&self) -> &Panel {
        &self.backbuffer
    }

    pub fn panel_mut(&mut self) -> &mut Panel {
        &mut self.backbuffer
    }

    /// Clears the internal backbuffer using the given `Cell` as a blank.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::io::{Error, Result};
    /// use rustty::{Terminal, Cell};
    ///
    /// # fn foo() -> Result<()> {
    ///
    /// let mut cell1 = Cell::default();
    /// cell1.set_ch('x');
    /// let mut cell2 = Cell::default();
    /// cell2.set_ch('y');
    ///
    /// let mut term = try!(Terminal::new());
    /// assert_eq!(term[(0, 0)].ch(), ' ');
    ///
    /// try!(term.clear(cell1));
    /// assert_eq!(term[(0, 0)].ch(), 'x');
    ///
    /// try!(term.clear(cell2));
    /// assert_eq!(term[(0, 0)].ch(), 'y');
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn clear(&mut self, cell: Cell) -> Result<()> {
        // Check whether the window has been resized; if it has then update and resize the buffers.
        if SIGWINCH_STATUS.compare_and_swap(true, false, Ordering::SeqCst) {
            try!(self.resize());
        }
        self.backbuffer.clear(cell);
        Ok(())
    }

    /// Resizes the buffers if the underlying terminal window size has changed, using the default
    /// `Cell` as a blank.
    ///
    /// This method will be called automatically on each call to `refresh()` or a `clear()`
    /// method.
    ///
    /// This method is guaranteed to resize the buffers if a call to `check_resize()` returns
    /// `true` and neither `refresh()` nor a `clear()` method has been called since.
    ///
    /// Returns `Some((cols, rows))` if the buffers were resized and `None` if no resize was
    /// performed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::io::{Error, Result};
    /// use rustty::Terminal;
    ///
    /// # fn foo() -> Result<()> {
    ///
    /// let mut term = try!(Terminal::new());
    ///
    /// // If new_size == Some(T) then T is the new size of the terminal.
    /// // If new_size == None then the terminal has not resized.
    /// let new_size = try!(term.try_resize());
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn try_resize(&mut self) -> Result<Option<(usize, usize)>> {
        if SIGWINCH_STATUS.compare_and_swap(true, false, Ordering::SeqCst) {
            try!(self.resize());
            return Ok(Some((self.cols(), self.rows())));
        }
        Ok(None)
    }

    pub fn poll_events<'a>(&'a mut self) -> Result<PollEvents<'a>> {
        // Poll for new events.
        try!(self.read_events(None));
        // Return a draining iterator over the eventbuffer.
        Ok(PollEvents(self.eventbuffer.drain(..)))
    }

    pub fn wait_events<'a>(&'a mut self, timeout: Duration) -> Result<WaitEvents<'a>> {
        // Wait for new events.
        try!(self.read_events(Some(timeout)));
        // Return a draining iterator over the eventbuffer.
        Ok(WaitEvents(self.eventbuffer.drain(..)))
    }

    // Sends the cursor to the specified position.
    fn send_cursor(&mut self, x: usize, y: usize) -> Result<()> {
        try!(self.outbuffer.write_all(&self.driver.get(DevFn::SetCursor(x, y))));
        Ok(())
    }

    // Sets the cursor to the specified coordinates and then writes the specified character.
    //
    // At the moment, wide characters are going to make things go very, very wrong...probably.
    fn send_char(&mut self, x: usize, y: usize, ch: char) -> Result<()> {
        try!(self.send_cursor(x, y));
        try!(write!(self.outbuffer, "{}", ch));
        Ok(())
    }

    // Clears the terminal with the default style.
    fn send_clear(&mut self) -> Result<()> {
        try!(self.outbuffer.write_all(&self.driver.get(DevFn::Reset)));
        try!(self.outbuffer.write_all(&self.driver.get(DevFn::Clear)));
        try!(self.flush());
        Ok(())
    }

    // Sends the style of the specified cell.
    fn send_style(&mut self, cell: Cell) -> Result<()> {
        try!(self.outbuffer.write_all(&self.driver.get(DevFn::Reset)));

        if cell.attrs().contains(BOLD) {
            try!(self.outbuffer.write_all(&self.driver.get(DevFn::Bold)));
        }
        if cell.attrs().contains(UNDERLINE) {
            try!(self.outbuffer.write_all(&self.driver.get(DevFn::Underline)));
        }
        if cell.attrs().contains(REVERSE) {
            try!(self.outbuffer.write_all(&self.driver.get(DevFn::Reverse)));
        }

        try!(self.write_sgr(cell.fg(), cell.bg()));
        Ok(())
    }

    // Writes colors to the outbuffer.
    fn write_sgr(&mut self, fgcol: Color, bgcol: Color) -> Result<()> {
        match fgcol {
            Color::Default => {}
            fgc @ _ => {
                try!(self.outbuffer.write_all(&self.driver.get(DevFn::SetFg(fgc.as_byte()))));
            }
        }
        match bgcol {
            Color::Default => {}
            bgc @ _ => {
                try!(self.outbuffer.write_all(&self.driver.get(DevFn::SetBg(bgc.as_byte()))));
            }
        }
        Ok(())
    }

    // Updates the size of the Terminal object to reflect that of the underlying terminal.
    fn resize(&mut self) -> Result<()> {
        let (cols, rows) = try!(self.tty.window_size());
        self.backbuffer.resize(cols, rows, Cell::default());
        self.frontbuffer.resize(cols, rows, Cell::default());
        self.frontbuffer.clear(Cell::default());
        try!(self.send_clear());
        Ok(())
    }

    // Attempts to read any available events from the terminal into the event buffer, waiting for
    // the specified timeout for input to become available.
    //
    // Returns the number of events read into the buffer.
    fn read_events(&mut self, timeout: Option<Duration>) -> Result<usize> {
        let nevts;
        let mut timeout_arg = if let Some(tout) = timeout {
            &cvt_duration(tout)
        } else {
            ptr::null()
        };

        let rawfd = self.tty.as_raw_fd();
        let nfds = rawfd + 1;

        let mut rfds: libc::fd_set = unsafe { mem::zeroed() };
        unsafe {
            libc::FD_SET(rawfd, &mut rfds);
        }

        // Because the sigwinch handler will interrupt select, if select returns EINTR we loop
        // and try again. All other errors will return normally.
        let start_inst = Instant::now();
        loop {
            let res = unsafe {
                libc::pselect(nfds,
                              &mut rfds,
                              ptr::null_mut(),
                              ptr::null_mut(),
                              timeout_arg,
                              ptr::null())
            };

            if res == -1 {
                let err = Error::last_os_error();

                if err.kind() == ErrorKind::Interrupted {
                    // If the window size has changed, push event onto the queue.
                    if SIGWINCH_STATUS.load(Ordering::SeqCst) {
                        self.eventbuffer.push_back(Event::Resize);
                    }
                    // If a timeout was specified, subtract elapsed time from it.
                    if let Some(orig_tout) = timeout {
                        let new_tout = orig_tout - start_inst.elapsed();
                        timeout_arg = &cvt_duration(new_tout);
                    }
                    // Loop and call `pselect` again.
                    continue;
                } else {
                    // Error other than EINTR, return to caller.
                    return Err(err);
                }
            } else {
                nevts = res;
                break;
            }
        }

        if nevts == 0 {
            // No input available. Return None.
            Ok(0)
        } else {
            // Input is available from the terminal.
            // Get an iterator of chars over the input stream.
            let mut buf = String::new();
            try!(self.tty.read_to_string(&mut buf));
            let mut n = 0;
            for ch in buf.chars() {
                // Push each character onto the event queue and increment the count.
                self.eventbuffer.push_back(Event::Key(ch));
                n += 1;
            }
            Ok(n)
        }
    }

    fn flush(&mut self) -> Result<()> {
        try!(self.tty.write_all(&self.outbuffer));
        self.outbuffer.clear();
        if thread::panicking() {
            let mut error = String::new();
            self.stderr_handle.read_to_string(&mut error).unwrap();
            print!("{}", error);
        }
        Ok(())
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        self.outbuffer.write_all(&self.driver.get(DevFn::ShowCursor)).unwrap();
        self.outbuffer.write_all(&self.driver.get(DevFn::Reset)).unwrap();
        self.outbuffer.write_all(&self.driver.get(DevFn::Clear)).unwrap();
        self.outbuffer.write_all(&self.driver.get(DevFn::ExitCa)).unwrap();
        self.flush().unwrap();
        SIGWINCH_STATUS.store(false, Ordering::SeqCst);
        RUSTTY_STATUS.store(false, Ordering::SeqCst);
    }
}

// Sigwinch handler to notify when window has resized.
extern "C" fn sigwinch_handler(_: i32) {
    SIGWINCH_STATUS.store(true, Ordering::SeqCst);
}

// Convenience function to convert `Duration` to `libc::timespec`.
fn cvt_duration(dur: Duration) -> libc::timespec {
    libc::timespec {
        tv_sec: dur.as_secs() as libc::time_t,
        tv_nsec: dur.subsec_nanos() as libc::c_long,
    }
}

pub struct PollEvents<'a>(vec_deque::Drain<'a, Event>);

impl<'a> Iterator for PollEvents<'a> {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

pub struct WaitEvents<'a>(vec_deque::Drain<'a, Event>);

impl<'a> Iterator for WaitEvents<'a> {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
