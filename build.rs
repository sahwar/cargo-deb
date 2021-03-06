use std::env;

// env::ARCH doesn't include full triple, and AFAIK there isn't a nicer way of getting the full triple
// (see lib.rs for the rest of this hack)
fn main() {
    let out = std::path::PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR")).join("default_target.rs");
    std::fs::write(out, env::var("TARGET").expect("TARGET")).unwrap();

    println!("cargo:rerun-if-changed=build.rs"); // optimization: avoid re-running this script
}
