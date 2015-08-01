pub mod terminal;
pub mod cellbuffer;
pub mod driver;
pub mod position;
pub mod input;
pub mod termctl;

macro_rules! write_all {
    ( $dst:expr, $src:expr ) => ( $dst.write_all($src) );
}
