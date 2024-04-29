extern crate cc;

fn main() {
    println!("cargo::rerun-if-changed=src/c/");
    cc::Build::new()
        .file("src/c/main.c")
        .file("src/c/windowlist.c")
        .compile("libwindowc.a");
    println!("cargo:rustc-link-lib=X11");
}
