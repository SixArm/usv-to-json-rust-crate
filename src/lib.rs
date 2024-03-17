use usv::*;
use std::convert::AsRef;

pub fn usv_to_json<
    //TODO S: AsRef<str> + Sized
>(
    usv: &str
) -> Result<String, serde_json::Error> {
    //TODO refactor to usv crate
    let nest: Vec<Vec<Vec<Vec<String>>>> = usv.files().map(|file|
        file.groups().map(|group|
            group.records().map(|record|
                record.units().map(|unit|
                    unit
                ).collect()
            ).collect()
        ).collect()
    ).collect();
    Ok(serde_json::to_string(&nest)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let usv = String::from("a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝␜i␟j␟␞k␟l␟␞␝m␟n␟␞o␟p␟␞␝␜");
        let json = String::from(r#"[[[["a","b"],["c","d"]],[["e","f"],["g","h"]]],[[["i","j"],["k","l"]],[["m","n"],["o","p"]]]]"#);    
        assert_eq!(usv_to_json(&usv).unwrap(), json);
    }

}
