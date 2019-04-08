#[macro_use]
extern crate lazy_static;
extern crate serde;
extern crate regex;
extern crate serde_regex;

use serde::Deserialize;
use serde_json;
use std::collections::HashMap;
use regex::Regex;

lazy_static! {
    static ref LANGUAGE_CODES: Vec<String> = vec![
        String::from("de"),
        String::from("en"),
        String::from("es"),
        String::from("et"),
        String::from("fi"),
        String::from("fr"),
        String::from("he"),
        String::from("id"),
        String::from("it"),
        String::from("ja"),
        String::from("nl"),
        String::from("no"),
        String::from("pl"),
        String::from("pt"),
        String::from("ro"),
        String::from("ru"),
        String::from("sv")
    ];
}

#[derive(Debug, PartialEq)]
pub enum Error {
    LanguageCodeNotSupported
}

#[derive(Deserialize, Debug)]
pub struct Token {
    tokens: Vec<String>,
    #[serde(with = "serde_regex")]
    full: Regex,
    canonical: String,
    #[serde(rename = "spanBoundaries")]
    span_boundaries: Option<u8>,
    #[serde(rename = "onlyLayers")]
    only_layers: Option<Vec<String>>,
    note: Option<String>,
    #[serde(rename = "type")]
    token_type: Option<String>,
    regex: Option<bool>
}

pub fn tokens(v: Vec<String>) -> Result<HashMap<String, Vec<Token>>, Error> {
    if v.is_empty() {
        return Ok(get_tokens(LANGUAGE_CODES.to_vec()))
    }
    for lc in &v {
        if !LANGUAGE_CODES.contains(lc) {
            return Err(Error::LanguageCodeNotSupported)
        }
    }
    Ok(get_tokens(v))
}

fn get_tokens(v: Vec<String>) -> HashMap<String, Vec<Token>> {
    let mut map = HashMap::new();
    for lc in &v {
        let tokens_str = import(lc);
        let parsed_tokens : Vec<Token> = serde_json::from_str(tokens_str)
            .expect("unable to parse JSON");
        map.insert(lc.clone(), parsed_tokens);
    }
    map
}

fn import(lc: &str) -> &str {
    match lc {
        "de" => include_str!("../tokens/de.json"),
        "en" => include_str!("../tokens/en.json"),
        "es" => include_str!("../tokens/es.json"),
        "et" => include_str!("../tokens/et.json"),
        "fi" => include_str!("../tokens/fi.json"),
        "fr" => include_str!("../tokens/fr.json"),
        "he" => include_str!("../tokens/he.json"),
        "id" => include_str!("../tokens/id.json"),
        "it" => include_str!("../tokens/it.json"),
        "ja" => include_str!("../tokens/ja.json"),
        "nl" => include_str!("../tokens/nl.json"),
        "no" => include_str!("../tokens/no.json"),
        "pl" => include_str!("../tokens/pl.json"),
        "pt" => include_str!("../tokens/pt.json"),
        "ro" => include_str!("../tokens/ro.json"),
        "ru" => include_str!("../tokens/ru.json"),
        "sv" => include_str!("../tokens/sv.json"),
        _ => panic!("token file import not set up for supported language code")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_tokens() {
        let lc_tokens = tokens(vec![String::from("de"), String::from("en")]).unwrap();
        assert_eq!(lc_tokens.len(), 2);
        assert!(lc_tokens.contains_key("de"));
        assert!(lc_tokens.contains_key("en"));

        let empty_lc = tokens(Vec::new()).unwrap();
        let every_lc = get_tokens(LANGUAGE_CODES.to_vec());
        assert_eq!(empty_lc.len(), every_lc.len());
    }

    #[test]
    #[should_panic(expected = "LanguageCodeNotSupported")]
    fn fail_tokens() {
        tokens(vec![String::from("zz")]).unwrap();
    }

    #[test]
    fn test_all_lcs() {
        let file_system_lcs = read_files();
        assert_eq!(LANGUAGE_CODES.len(), file_system_lcs.len());
        // TODO test values as well as length
    }

    #[test]
    fn test_get_tokens() {
        let lc_tokens = get_tokens(vec![String::from("de"), String::from("en")]);
        assert_eq!(lc_tokens.len(), 2);
        assert!(lc_tokens.contains_key("de"));
        assert!(lc_tokens.contains_key("en"));
    }

    #[test]
    fn test_token_values() {
        let token_types = vec![
            String::from("box"),
            String::from("cardinal"),
            String::from("number"),
            String::from("ordinal"),
            String::from("unit"),
            String::from("way")
        ];
        let token_map = tokens(Vec::new()).unwrap();

        for lc in token_map.values() {
            for tk in lc {
                assert!(tk.tokens.len() > 0);
                match &tk.only_layers {
                    Some(l) => {
                        assert_eq!(l[0], "address");
                        assert_eq!(l.len(), 1);
                    },
                    _ => (),
                }
                match &tk.token_type {
                    Some(t) => assert!(token_types.contains(t)),
                    _ => (),
                }

            }
        }
    }

    fn read_files() -> Vec<String> {
        let mut lcs = Vec::new();
        for entry in fs::read_dir("./tokens").unwrap() {
            let file_name = entry.unwrap().file_name().into_string().unwrap();
            let file_components: Vec<&str> = file_name.split(".").collect();
            if file_components[1] == "json" {
                lcs.push(file_components[0].to_owned());
            }
        }
        lcs
    }
}
