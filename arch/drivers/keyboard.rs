use std::cast;
use std::option::{Option, Some, None};
use std::str::StrSlice;
use extra::bitv::SmallBitv;
use cpu;
use cpu::io;

//
// Constants
//
static KBSTATP: u8  = 0x64; // kbd controller status port(I)
static KBS_DIB: u8  = 0x01; // kbd data in buffer
static KBDATAP: u8  = 0x60; // kbd data port(I)

// Special keycodes
static KEY_HOME: u8 = 0xE0;
static KEY_END: u8  = 0xE1;
static KEY_UP: u8   = 0xE2;
static KEY_DN: u8   = 0xE3;
static KEY_LF: u8   = 0xE4;
static KEY_RT: u8   = 0xE5;
static KEY_PGUP: u8 = 0xE6;
static KEY_PGDN: u8 = 0xE7;
static KEY_INS: u8  = 0xE8;
static KEY_DEL: u8  = 0xE9;

// Key modifier
static MOD_NO: uint         = 0;
static MOD_SHIFT: uint      = 1;
static MOD_CTL: uint        = 2;
static MOD_ALT: uint        = 3;
static MOD_CAPSLOCK: uint   = 4;
static MOD_NUMLOCK: uint    = 5;
static MOD_SCROLLLOCK: uint = 6;
static MOD_ESC: uint        = 7;

//
// Keyboard maps
//
static qwerty_normal: &'static str = "\
\x00\x1B123456\
7890-=\x08\t\
qwertyui\
op[]\n\x00as\
dfghjkl;\
'`\x00\\zxcv\
bnm,./\x00*\
\x00 \x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x007\
89-456+1\
230.\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
";

static qwerty_shift: &'static str = "\
\x00\x1B!@#$%^\
&*()_+\x08\t\
QWERTYUI\
OP{}\n\x00AS\
DFGHJKL:\
\"~\x00|ZXCV\
BNM<>?\x00*\
\x00 \x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x007\
89-456+1\
230.\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
";

static qwerty_ctl: &'static str = "
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\
";

static RING_BUF_SIZE: uint = 512;
struct ring_buf {
    buf: [u8, ..RING_BUF_SIZE],
    rpos: uint,
    wpos: uint,
}

// The circular keyboard input buffer
static mut kbd_buf: ring_buf = ring_buf {
    buf: [0u8, ..RING_BUF_SIZE],
    rpos: 0,
    wpos: 0,
};

pub fn init() {
    unsafe {
        init_kbdmaps();
        cpu::irq_setmask_8259A(cpu::IRQ_MASK_8259A & !(1<<1));
    }
}

pub fn getchar() -> u8 {
    loop {
        let c = getc();
        if c != 0 { return c; }
    }
}

pub fn getc() -> u8 {
    unsafe {
        kbd_intr();

        if kbd_buf.rpos != kbd_buf.wpos {
            let c = kbd_buf.buf[kbd_buf.rpos];
            kbd_buf.rpos = (kbd_buf.rpos + 1) % RING_BUF_SIZE;
            c
        } else {
            0
        }
    }
}

pub unsafe fn kbd_intr() {
    loop {
        match keypress() {
            Some(c) => if c != 0 {
                kbd_buf.buf[kbd_buf.wpos] = c;
                kbd_buf.wpos = (kbd_buf.wpos + 1) % RING_BUF_SIZE;
            },
            None    => return,
        }
    }
}

static mut key_modifier: SmallBitv = SmallBitv { bits: 0 };

unsafe fn keypress() -> Option<u8> {
    if io::inb(KBSTATP as u16) & KBS_DIB == 0 {
        return None;
    }

    let mut data = io::inb(KBDATAP as u16);

    match data {
        0xE0 => {
            key_modifier.set(MOD_ESC, true);
            Some(0)
        },
        _ if data & 0x80 != 0   => {
            // Key released
            data = if key_modifier.get(MOD_ESC) { data } else { data & 0x7F };
            key_modifier.set(shift_code(data) as uint, false);
            key_modifier.set(MOD_ESC, false);
            Some(0)
        },
        _  => {
            if key_modifier.get(MOD_ESC) {
                data |= 0x80;
                key_modifier.set(MOD_ESC, false);
            }

            key_modifier.set(shift_code(data), true);
            key_modifier.set(toggle_code(data),
                !key_modifier.get(toggle_code(data)));

            data = if key_modifier.get(MOD_CTL) {
                qwerty_ctl[data]
            } else if key_modifier.get(MOD_SHIFT) {
                qwerty_shift[data]
            } else {
                qwerty_normal[data]
            };
            
            if key_modifier.get(MOD_CAPSLOCK) {
                data = match data as char {
                    'a'..'z'    => data + 'A' as u8 - 'a' as u8,
                    'A'..'Z'    => data + 'a' as u8 - 'A' as u8,
                    _           => data
                }
            }
            Some(data)
        }
    }
}

#[inline]
fn shift_code(c: u8) -> uint {
    match c {
        0x1D | 0x9D => MOD_CTL,
        0x2A | 0x36 => MOD_SHIFT,
        0x38 | 0xB8 => MOD_ALT,
        _           => MOD_NO,
    }
}

#[inline]
fn toggle_code(c: u8) -> uint {
    match c {
        0x3A    => MOD_CAPSLOCK,
        0x45    => MOD_NUMLOCK,
        0x46    => MOD_SCROLLLOCK,
        _       => MOD_NO,
    }
}

unsafe fn init_kbdmaps() {
    let normal_map: &mut [u8] = cast::transmute(qwerty_normal.as_bytes());
    let shift_map: &mut [u8] = cast::transmute(qwerty_shift.as_bytes());
    let ctl_map: &mut [u8] = cast::transmute(qwerty_ctl.as_bytes());

    normal_map[0x9C] = '\n' as u8;
    normal_map[0xB5] = '/' as u8;
    normal_map[0xC7] = KEY_HOME;
    normal_map[0xC8] = KEY_UP;
    normal_map[0xC9] = KEY_PGUP;
    normal_map[0xCB] = KEY_LF;
    normal_map[0xCD] = KEY_RT;
    normal_map[0xCF] = KEY_END;
    normal_map[0xD0] = KEY_DN;
    normal_map[0xD1] = KEY_PGDN;
    normal_map[0xD2] = KEY_INS;
    normal_map[0xD3] = KEY_DEL;

    shift_map[0x9C] = '\n' as u8;
    shift_map[0xB5] = '/' as u8;
    shift_map[0xC7] = KEY_HOME;
    shift_map[0xC8] = KEY_UP;
    shift_map[0xC9] = KEY_PGUP;
    shift_map[0xCB] = KEY_LF;
    shift_map[0xCD] = KEY_RT;
    shift_map[0xCF] = KEY_END;
    shift_map[0xD0] = KEY_DN;
    shift_map[0xD1] = KEY_PGDN;
    shift_map[0xD2] = KEY_INS;
    shift_map[0xD3] = KEY_DEL;

    ctl_map[0xB5] = '/' as u8;
    ctl_map[0x97] = KEY_HOME;
    ctl_map[0xC8] = KEY_UP;
    ctl_map[0xC9] = KEY_PGUP;
    ctl_map[0xCB] = KEY_LF;
    ctl_map[0xCD] = KEY_RT;
    ctl_map[0xCF] = KEY_END;
    ctl_map[0xD0] = KEY_DN;
    ctl_map[0xD1] = KEY_PGDN;
    ctl_map[0xD2] = KEY_INS;
    ctl_map[0xD3] = KEY_DEL;    
}
