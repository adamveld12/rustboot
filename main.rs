#![no_std]
#![allow(ctypes)]

extern crate core;

use core::iter::Iterator;
use core::str::StrExt;
use core::option::Option;

use core::kinds::Copy;

impl Copy for Color {}

enum Color {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Pink       = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    LightPink  = 13,
    Yellow     = 14,
    White      = 15,
}

struct IntRange {
    cur: int,
    max: int
}

impl IntRange {
    fn next(&mut self) -> Option<int> {
        if self.cur < self.max {
            self.cur += 1;
            Option::Some(self.cur - 1)
        } else {
            Option::None
        }
    }
}

fn range(lo: int, hi: int) -> IntRange {
    IntRange { cur: lo, max: hi }
}

fn clear_screen(background: Color) {
    let mut r = range(0, 80 * 25);
    loop {
        match r.next() {
            Option::Some(x) => {
                unsafe {
                    *((0xb8000 + x * 2) as *mut u16) = (background as u16) << 12;
                }
            },
            Option::None =>{break}
        }
    }
}

fn print_string(string: &str) {
    let mut bytes = string.bytes();
    let mut count = 0;
    loop {
        match bytes.next() {
            Option::Some(x) => {
                unsafe {
                    *((0xb8000 + count) as *mut u8) = x as u8;
                    *((0xb8000 + count + 1) as *mut u8) = 0x0f;
                }
                count = count + 1;
            },
            Option::None =>{break}
        }
    }
}

#[no_mangle]
#[no_split_stack]
pub fn main() {
    clear_screen(Color::LightRed);
    print_string("Hello world");
}
