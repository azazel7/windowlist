extern crate libc;

use libc::c_char;

extern "C" {
    pub fn main_c(argc: i32, s: *const u8 ) -> i32;
}

