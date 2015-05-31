use std::fmt;
use std::io;
use std::io::Read;
use std::error::Error;
use std::str;

/// Identical implementation of io::Chars, but available in stable.
pub struct CharStream<T: Read> {
    inner: T,
}

impl<T: Read> CharStream<T> {
    pub fn from_reader(r: T) -> CharStream<T> {
        CharStream { inner: r }
    }
}

#[derive(Debug)]
pub enum CharStreamError {
    NotUtf8,
    Other(io::Error),
}

impl<R: Read> Iterator for CharStream<R> {
    type Item = Result<char, CharStreamError>;

    fn next(&mut self) -> Option<Result<char, CharStreamError>> {
        let mut buf = [0];
        let first_byte = match self.inner.read(&mut buf) {
            Ok(0) => return None,
            Ok(..) => buf[0],
            Err(e) => return Some(Err(CharStreamError::Other(e))),
        };
        let width = utf8_char_width(first_byte);
        if width == 1 { return Some(Ok(first_byte as char)) }
        if width == 0 { return Some(Err(CharStreamError::NotUtf8)) }
        let mut buf = [first_byte, 0, 0, 0];
        {
            let mut start = 1;
            while start < width {
                match self.inner.read(&mut buf[start..width]) {
                    Ok(0) => return Some(Err(CharStreamError::NotUtf8)),
                    Ok(n) => start += n,
                    Err(e) => return Some(Err(CharStreamError::Other(e))),
                }
            }
        }
        Some(match str::from_utf8(&buf[..width]).ok() {
            Some(s) => Ok(s.chars().nth(0).unwrap()),
            None => Err(CharStreamError::NotUtf8),
        })
    }
}

impl Error for CharStreamError {
    fn description(&self) -> &str {
        match *self {
            CharStreamError::NotUtf8 => "invalid utf8 encoding",
            CharStreamError::Other(ref e) => Error::description(e),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            CharStreamError::NotUtf8 => None,
            CharStreamError::Other(ref e) => e.cause(),
        }
    }
}

impl fmt::Display for CharStreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CharStreamError::NotUtf8 => {
                "byte stream did not contain valid utf8".fmt(f)
            }
            CharStreamError::Other(ref e) => e.fmt(f),
        }
    }
}

static UTF8_CHAR_WIDTH: [u8; 256] = [
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x1F
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x3F
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x5F
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x7F
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, // 0x9F
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, // 0xBF
    0,0,2,2,2,2,2,2,2,2,2,2,2,2,2,2,
    2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2, // 0xDF
    3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3, // 0xEF
    4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0, // 0xFF
];

#[inline]
fn utf8_char_width(b: u8) -> usize {
    return UTF8_CHAR_WIDTH[b as usize] as usize;
}
