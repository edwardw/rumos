use std::io;
use std::str;
use extra::term;
use arch::drivers::vga;

pub struct kConsole;

impl io::Writer for kConsole {
    fn write(&mut self, buf: &[u8]) {
        vga::puts(str::from_utf8(buf), term::color::WHITE);
    }
}
