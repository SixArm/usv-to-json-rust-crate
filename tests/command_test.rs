mod common; use common::*;
use std::process::Command;

#[test]
fn command() {
    let input = "a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝␜i␟j␟␞k␟l␟␞␝m␟n␟␞o␟p␟␞␝␜";
    let expect = String::from(r#"[[[["a","b"],["c","d"]],[["e","f"],["g","h"]]],[[["i","j"],["k","l"]],[["m","n"],["o","p"]]]]"#) + "\n";
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io_str_to_string(&mut command, input);
    assert_eq!(actual, expect);
}
