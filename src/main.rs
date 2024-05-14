use windowlist::config;
use windowlist::configuration::Configuration;

use std::env;


use std::collections::HashMap;
use std::ffi::CString;
use std::mem;

fn main() {
    println!(
        "Hello, world! {} {}",
        mem::size_of::<Vec<CString>>(),
        mem::size_of::<HashMap<CString, CString>>()
    );
    let args: Vec<String> = env::args().collect();
    let Some(wid) = args.get(0) else {eprintln!("No path."); return;};

    let configuration = Configuration::new("config.toml".to_string());
    unsafe {
        config = configuration;
    }

    unsafe {
        eprintln!("Config : {:?}", windowlist::config);
        let argv = wid.as_ptr();
        windowlist::main_c(1, argv);
    }
}
