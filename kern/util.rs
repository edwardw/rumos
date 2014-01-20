use std::cast;
use std::io;
use std::str;
use extra::term;
use arch::drivers::vga;

pub fn get_kconsole() -> &mut io::Writer {
    unsafe { cast::transmute(&kconsole as &io::Writer) }
}

struct Stdout;
static kconsole: Stdout = Stdout;
impl io::Writer for Stdout {
    fn write(&mut self, buf: &[u8]) {
        vga::puts(str::from_utf8(buf), term::color::WHITE);
    }
}
