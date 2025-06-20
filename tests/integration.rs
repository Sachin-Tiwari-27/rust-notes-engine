#[test]

fn test_cli_add() {
    use std::process::Command;
    let output = Command::new("cargo")
        .args(["run", "--", "add", "Test CLI", "CLI Body", "Work"])
        .output()
        .expect("failed to run");
    assert!(String::from_utf8_lossy(&output.stdout).contains("Note added"));
}
