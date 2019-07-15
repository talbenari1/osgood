use std::process::Command;

#[test]
fn format() {
    let child = Command::new("cargo")
        .args(&["fmt", "--all", "--", "--check"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .status()
        .unwrap();
    assert!(child.success());
}
