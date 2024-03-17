#[path = "testing.rs"]
mod testing;
use testing::*;

use std::process::{Command, Stdio};
use std::io::Write;

#[test]
fn command() {
    let input = String::from("a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝␜i␟j␟␞k␟l␟␞␝m␟n␟␞o␟p␟␞␝␜");
    let expect = String::from(r#"[[[["a","b"],["c","d"]],[["e","f"],["g","h"]]],[[["i","j"],["k","l"]],[["m","n"],["o","p"]]]]"#) + "\n";

    // Given
    let mut command = Command::new(&*COMMAND_OS)
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()
    .expect("command");

    // When
    let child_stdin = command.stdin.as_mut().expect("child_stdin");
    child_stdin.write_all(input.as_bytes()).expect("write_all");
    #[allow(dropping_references)]
    let _ = drop(child_stdin);

    // Then
    let output = command.wait_with_output().expect("wait_with_output");
    let actual: String = String::from_utf8(output.stdout).unwrap();
    assert_eq!(actual, expect);
}
