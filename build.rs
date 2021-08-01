use std::{path::PathBuf, fs, env};


fn main() {
    let path = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    fs::write(path.join("generated.rs"), "pub fn bar () {}").unwrap();
}
