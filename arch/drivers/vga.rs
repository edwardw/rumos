use std::prelude::*;
use cpu;
use cpu::io;

#[packed]
pub struct character {
    char: u8,
    attr: u8,
}

static SCREEN_ROWS: uint = 25;
static SCREEN_COLS: uint = 80;
static SCREEN_SIZE: uint = SCREEN_ROWS*SCREEN_COLS;
type screen_buf = [character, ..SCREEN_SIZE];
static screen: *mut screen_buf = 0xB8000 as *mut screen_buf;
static mut cur_pos: uint = 0;

pub fn init() {
    unsafe {
        cur_pos = cursor_pos();
    }
}

pub fn puts(string: &str, attr: term::color::Color) {
    stdio::puts(string, attr, putc, new_line);
}

pub fn putc(c: char, attr: term::color::Color) {
    unsafe {
        put_char(cur_pos, character{char: c as u8, attr: attr as u8});
        cursor_move(1);
    }
}

pub fn new_line() {
    unsafe {
        cursor_move(SCREEN_COLS - cur_pos % SCREEN_COLS);
    }
}

#[inline]
unsafe fn cursor_move(delta: uint) {
    cur_pos += delta;
    if cur_pos >= SCREEN_SIZE {
        cpu::memmove(mem_ptr_of(0, 0), mem_ptr_of(1, 0),
            (SCREEN_SIZE - SCREEN_COLS) * mem::size_of::<character>());
        let mut i = SCREEN_SIZE - SCREEN_COLS;
        while i < SCREEN_SIZE {
            put_char(i, character{char: ' ' as u8, attr: term::color::BLACK as u8});
            i += 1;
        };
        cur_pos -= SCREEN_COLS;
    }
    cursor_to(cur_pos);
}

#[inline]
unsafe fn put_char(pos: uint, c: character) {
    (*screen)[pos] = c;
}

#[inline]
unsafe fn cursor_pos() -> uint {
    let mut pos: uint;
    io::outb(0x3D4, 14);
    pos = (io::inb(0x3D5) as uint) << 8;
    io::outb(0x3D4, 15);
    pos |= io::inb(0x3D5) as uint;
    pos
}

#[inline]
unsafe fn cursor_to(pos: uint) {
    io::outb(0x3D4, 14);
    io::outb(0x3D5, (pos >> 8) as u8);
    io::outb(0x3D4, 15);
    io::outb(0x3D5, pos as u8);
}

#[inline]
unsafe fn mem_ptr_of(row: uint, col: uint) -> uint {
    screen as uint +
    row * SCREEN_COLS * mem::size_of::<character>() +
    col * mem::size_of::<character>()
}
