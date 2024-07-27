//use std::process::Command;
use assert_cmd::Command;

#[test]
fn works() {
    //let mut cmd = Command::new("");
    let mut cmd = Command::cargo_bin("shzgptr").unwrap();
    cmd.assert().success();
}
