use click_actions::XorgConnection;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let Some(wid) = args.get(1) else {eprintln!("No enough args."); return;};
    let Some(wid) = u64::from_str_radix(wid.trim_start_matches("0x"), 16).ok() else {eprintln!("{wid} is not a number."); return;};

    let conn = XorgConnection::new();
    conn.close(wid);
}
