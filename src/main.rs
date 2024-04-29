use std::env;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    let Some(wid) = args.get(0) else {eprintln!("No path."); return;};
    unsafe {
       let argv = wid.as_ptr();
       windowlist::main_c(1, argv);
    }
}
