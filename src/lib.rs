use serde::Deserialize;
use serde_json;
use std::collections::HashMap;

macro_rules! iterable_enum {
    ($name:ident { $($variant:ident),* })   => (
        #[derive(Debug, PartialEq, Eq, Hash, Clone)]
        pub enum $name { $($variant),* }

        impl $name {
            fn iter() -> Iter {
                Iter(None)
            }
        }

        struct Iter(Option<$name>);

        impl Iterator for Iter {
            type Item = $name;

            fn next(&mut self) -> Option<Self::Item> {
                match self.0 {
                    None                    => $( { self.0 = Some($name::$variant); Some($name::$variant) },
                    Some($name::$variant)   => )* None,
                }
            }
        }
    );
}

iterable_enum!{
    CC {
        De,
        En,
        Es,
        Fi
    }
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
    let ccs = if v.is_empty() {
        every_cc()
    } else {
        to_cc(v).unwrap()
    };
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

fn every_cc() -> Vec<CC> {
    let mut ccs = Vec::new();
    for cc in CC::iter() {
        ccs.push(cc);
    }
    ccs
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_cc() {
        assert_eq!(to_cc(vec!["de"]).unwrap(), vec![CC::De]);
        assert_eq!(to_cc(vec!["de", "en"]).unwrap(), vec![CC::De, CC::En]);
    }

    #[test]
    #[should_panic(expected = "CountryCodeNotSupported")]
    fn fail_to_cc() {
        to_cc(vec!["zz"]).unwrap();
    }

    #[test]
    fn test_every_cc() {
        let ccs = every_cc();
        assert_eq!(ccs.len(), 4);
    }

    #[test]
    fn test_get_tokens() {
        let tokens = get_tokens(vec![CC::De, CC::En]);
        assert_eq!(tokens.len(), 2);
        assert!(tokens.contains_key(&CC::De));
        assert!(tokens.contains_key(&CC::En));
    }

    #[test]
    fn test_tokens() {
        let tokens = tokens(vec!["de", "en"]);
        assert_eq!(tokens.len(), 2);
        assert!(tokens.contains_key(&CC::De));
        assert!(tokens.contains_key(&CC::En));
    }

    #[test]
    fn test_tokens_empty() {
        let tokens = tokens(Vec::new());
        assert_eq!(tokens.len(), 4);
    }

    #[test]
    #[should_panic(expected = "CountryCodeNotSupported")]
    fn fail_tokens() {
        tokens(vec!["zz"]);
    }
}
