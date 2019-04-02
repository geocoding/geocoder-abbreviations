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

fn main() {
    let v = vec!["de", "en", "es", "fi"];
    let vector_CCs = cc_to_CC(v).unwrap();
    let hashmap_tokens = construct_tokens(vector_CCs);
}

fn cc_to_CC(v: Vec<&str>) -> Result<Vec<CC>, Error> {
    let mut v_CC = Vec::new();
    for i in &v {
        match *i {
            "de" => v_CC.push(CC::De),
            "en" => v_CC.push(CC::En),
            "es" => v_CC.push(CC::Es),
            "fi" => v_CC.push(CC::Fi),
            _ => return Err(Error::CountryCodeNotSupported)
        }
    }
    Ok(v_CC)
}

fn construct_tokens(v: Vec<CC>) -> HashMap<CC, Vec<Token>> {
    let mut token_map = HashMap::new();
    for i in &v {
        let tokens = import_tokens(i);
        let parsed_tokens : Vec<Token> = serde_json::from_str(&tokens)
            .expect("unable to parse JSON");
        token_map.insert(i.clone(), parsed_tokens);
    }
    token_map
}

fn import_tokens(cc: &CC) -> String {
    match cc {
        CC::De => include_str!("../tokens/de.json").to_string(),
        CC::En => include_str!("../tokens/en.json").to_string(),
        CC::Es => include_str!("../tokens/es.json").to_string(),
        CC::Fi => include_str!("../tokens/fi.json").to_string()
    }
}
