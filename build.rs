use std::process::Command;

fn main() {
    let output = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .expect("Git command ran unsuccessfully.");

    let git_hash = String::from_utf8(output.stdout).unwrap();

    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
}
