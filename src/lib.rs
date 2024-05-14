extern crate libc;
pub mod configuration;

use crate::configuration::Configuration;


#[no_mangle]
pub extern "C" fn hello_from_rust() {
    println!("Hello from Rust!");
    unsafe { eprintln!("Config : {:?}", config) };
}
extern "C" {
    pub static mut config: Configuration;
    pub fn main_c(argc: i32, s: *const u8) -> i32;
}
