use std::env::temp_dir;

fn main() {
    let dir = temp_dir();
    let canonical_path = dir.as_path().canonicalize().expect("canonical path");
    dbg!(canonical_path);
}
