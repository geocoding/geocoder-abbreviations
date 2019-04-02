use serde::Deserialize;
use serde_json;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum CC {
    De,
    En,
    Es,
    Fi
}

#[derive(Debug, PartialEq)]
pub enum Error {
    CountryCodeNotSupported
}

#[derive(Deserialize, Debug)]
pub struct Token {
    tokens: Vec<String>,
    full: String,
    canonical: String,
    #[serde(rename = "onlyLayers")]
    only_layers: Option<Vec<String>>,
    note: Option<String>,
    #[serde(rename = "type")]
    token_type:  Option<String>,
}

pub fn tokens(v: Vec<&str>) -> HashMap<CC, Vec<Token>> {
    let ccs = to_cc(v).unwrap();
    get_tokens(ccs)
}

fn to_cc(v: Vec<&str>) -> Result<Vec<CC>, Error> {
    let mut ccs = Vec::new();
    for i in &v {
        match *i {
            "de" => ccs.push(CC::De),
            "en" => ccs.push(CC::En),
            "es" => ccs.push(CC::Es),
            "fi" => ccs.push(CC::Fi),
            _ => return Err(Error::CountryCodeNotSupported)
        }
    }
    Ok(ccs)
}

pub fn get_tokens(v: Vec<CC>) -> HashMap<CC, Vec<Token>> {
    let mut map = HashMap::new();
    for i in &v {
        let tokens = import(i);
        let parsed_tokens : Vec<Token> = serde_json::from_str(tokens)
            .expect("unable to parse JSON");
        map.insert(i.clone(), parsed_tokens);
    }
    map
}

fn import(cc: &CC) -> &str {
    match cc {
        CC::De => include_str!("../tokens/de.json"),
        CC::En => include_str!("../tokens/en.json"),
        CC::Es => include_str!("../tokens/es.json"),
        CC::Fi => include_str!("../tokens/fi.json")
    }
}
