#[macro_use]
extern crate lazy_static;
extern crate serde;
extern crate regex;
extern crate fancy_regex;
extern crate alphanumeric_sort;

use serde::Deserialize;
use serde_json;
use std::collections::HashMap;
use fancy_regex::Regex;

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

#[derive(Deserialize, Debug, Clone)]
struct InToken {
    tokens: Vec<String>,
    full: String,
    canonical: String,
    note: Option<String>,
    #[serde(rename = "onlyCountries")]
    only_countries: Option<Vec<String>>,
    #[serde(rename = "onlyLayers")]
    only_layers: Option<Vec<String>>,
    #[serde(rename = "preferFull")]
    prefer_full: Option<bool>,
    regex: Option<bool>,
    #[serde(rename = "skipBoundaries")]
    skip_boundaries: Option<bool>,
    #[serde(rename = "skipDiacriticStripping")]
    skip_diacritic_stripping: Option<bool>,
    #[serde(rename = "spanBoundaries")]
    span_boundaries: Option<u8>,
    #[serde(rename = "type")]
    token_type: Option<String>,
}

pub struct OutToken {
    tokens: Vec<String>,
    full: BasicToken,
    canonical: String,
    note: Option<String>,
    only_countries: Option<Vec<String>>,
    only_layers: Option<Vec<String>>,
    prefer_full: Option<bool>,
    regex: Option<bool>,
    skip_boundaries: Option<bool>,
    skip_diacritic_stripping: Option<bool>,
    span_boundaries: Option<u8>,
    token_type: Option<String>,
}

impl OutToken {
    fn new(input: InToken) -> OutToken {
        OutToken {
            tokens: input.tokens,
            full: match input.regex {
                Some(true) => BasicToken::Regex(Regex::new(&input.full).unwrap()),
                _ => BasicToken::String(input.full),
            },
            canonical: input.canonical,
            note: input.note,
            only_countries: input.only_countries,
            only_layers: input.only_layers,
            prefer_full: input.prefer_full,
            regex: input.regex,
            skip_boundaries: input.skip_boundaries,
            skip_diacritic_stripping: input.skip_diacritic_stripping,
            span_boundaries: input.span_boundaries,
            token_type: input.token_type,
        }
    }
}

pub enum BasicToken {
   String(String),
   Regex(Regex)
}


pub fn config(v: Vec<String>) -> Result<HashMap<String, Vec<OutToken>>, Error> {
    if v.is_empty() {
        return Ok(prepare(LANGUAGE_CODES.to_vec()))
    }
    for lc in &v {
        if !LANGUAGE_CODES.contains(lc) {
            return Err(Error::LanguageCodeNotSupported)
        }
    }
    Ok(prepare(v))
}

fn prepare(v: Vec<String>) -> HashMap<String, Vec<OutToken>> {
    let mut map = HashMap::new();
    for lc in &v {
        let parsed : Vec<InToken> = serde_json::from_str(import(lc))
            .expect("unable to parse token JSON");
        let mut tokens = Vec::new();
        for tk in &parsed {
            tokens.push(OutToken::new(tk.clone()));
        }
        map.insert(lc.clone(), tokens);
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
    fn test_config() {
        let lcs = config(vec![String::from("de"), String::from("en")]).unwrap();
        assert_eq!(lcs.len(), 2);
        assert!(lcs.contains_key("de"));
        assert!(lcs.contains_key("en"));

        let empty_lc = config(Vec::new()).unwrap();
        let every_lc = prepare(LANGUAGE_CODES.to_vec());
        assert_eq!(empty_lc.len(), every_lc.len());
        for lc in LANGUAGE_CODES.to_vec() {
            assert!(empty_lc.contains_key(&lc));
        }
    }

    #[test]
    #[should_panic(expected = "LanguageCodeNotSupported")]
    fn fail_config() {
        config(vec![String::from("zz")]).unwrap();
    }

    #[test]
    fn test_all_lcs() {
        let mut fs_lcs = read_files();
        alphanumeric_sort::sort_str_slice(&mut fs_lcs);
        assert_eq!(LANGUAGE_CODES.to_vec(), fs_lcs);
    }

    #[test]
    fn test_prepare() {
        let lcs = prepare(vec![String::from("de"), String::from("en")]);
        assert_eq!(lcs.len(), 2);
        assert!(lcs.contains_key("de"));
        assert!(lcs.contains_key("en"));
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
        let map = config(Vec::new()).unwrap();

        for lc in map.values() {
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
