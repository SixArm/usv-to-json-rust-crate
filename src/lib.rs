use usv::*;

//TODO Handle ESC and EOT
pub fn usv_to_json<
    //TODO S: AsRef<str> + Sized
>(
    usv: &str
) -> Result<String, serde_json::Error> {
    if usv.contains("\u{001C}") || usv.contains("␜") {
        let files: Files = usv.files().collect();
        return Ok(serde_json::to_string(&files)?)
    }
    if usv.contains("\u{001D}") || usv.contains("␝") {
        let groups: Groups = usv.groups().collect();
        return Ok(serde_json::to_string(&groups)?)
    }
    if usv.contains("\u{001E}") || usv.contains("␞") {
        let records: Records = usv.records().collect();
        return Ok(serde_json::to_string(&records)?)
    }
    if usv.contains("\u{001F}") || usv.contains("␟") {
        let units: Units = usv.units().collect();
        return Ok(serde_json::to_string(&units)?)
    }
    Err(serde::de::Error::custom(usv))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn units_test() {
        let usv = String::from("a␟b␟");
        let json = String::from(r#"["a","b"]"#);
        assert_eq!(usv_to_json(&usv).unwrap(), json);
    }

    #[test]
    fn records_test() {
        let usv = String::from("a␟b␟␞c␟d␟␞");
        let json = String::from(r#"[["a","b"],["c","d"]]"#);
        assert_eq!(usv_to_json(&usv).unwrap(), json);
    }

    #[test]
    fn groups_test() {
        let usv = String::from("a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝");
        let json = String::from(r#"[[["a","b"],["c","d"]],[["e","f"],["g","h"]]]"#);
        assert_eq!(usv_to_json(&usv).unwrap(), json);
    }

    #[test]
    fn files_test() {
        let usv = String::from("a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝␜i␟j␟␞k␟l␟␞␝m␟n␟␞o␟p␟␞␝␜");
        let json = String::from(r#"[[[["a","b"],["c","d"]],[["e","f"],["g","h"]]],[[["i","j"],["k","l"]],[["m","n"],["o","p"]]]]"#);
        assert_eq!(usv_to_json(&usv).unwrap(), json);
    }

}
