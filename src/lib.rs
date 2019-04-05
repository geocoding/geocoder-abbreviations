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
    LC {
        De,
        En,
        Es,
        Et,
        Fi,
        Fr,
        He,
        Id,
        It,
        Ja,
        Nl,
        No,
        Pl,
        Pt,
        Ro,
        Ru,
        Sv
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    LanguageCodeNotSupported
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

pub fn tokens(v: Vec<String>) -> HashMap<LC, Vec<Token>> {
    let lcs = if v.is_empty() {
        all_lcs()
    } else {
        to_lc(v).unwrap()
    };
    get_tokens(lcs)
}

fn all_lcs() -> Vec<LC> {
    let mut lcs = Vec::new();
    for lc in LC::iter() {
        lcs.push(lc);
    }
    lcs
}

fn to_lc(v: Vec<String>) -> Result<Vec<LC>, Error> {
    let mut lcs = Vec::new();
    for lc in &v {
        match lc.as_ref() {
            "de" => lcs.push(LC::De),
            "en" => lcs.push(LC::En),
            "es" => lcs.push(LC::Es),
            "et" => lcs.push(LC::Et),
            "fi" => lcs.push(LC::Fi),
            "fr" => lcs.push(LC::Fr),
            "he" => lcs.push(LC::He),
            "id" => lcs.push(LC::Id),
            "it" => lcs.push(LC::It),
            "ja" => lcs.push(LC::Ja),
            "nl" => lcs.push(LC::Nl),
            "no" => lcs.push(LC::No),
            "pl" => lcs.push(LC::Pl),
            "pt" => lcs.push(LC::Pt),
            "ro" => lcs.push(LC::Ro),
            "ru" => lcs.push(LC::Ru),
            "sv" => lcs.push(LC::Sv),
            _ => return Err(Error::LanguageCodeNotSupported)
        }
    }
    Ok(lcs)
}

fn get_tokens(v: Vec<LC>) -> HashMap<LC, Vec<Token>> {
    let mut map = HashMap::new();
    for lc in &v {
        let tokens_str = import(lc);
        let parsed_tokens : Vec<Token> = serde_json::from_str(tokens_str)
            .expect("unable to parse JSON");
        map.insert(lc.clone(), parsed_tokens);
    }
    map
}

fn import(lc: &LC) -> &str {
    match lc {
        LC::De => include_str!("../tokens/de.json"),
        LC::En => include_str!("../tokens/en.json"),
        LC::Es => include_str!("../tokens/es.json"),
        LC::Et => include_str!("../tokens/et.json"),
        LC::Fi => include_str!("../tokens/fi.json"),
        LC::Fr => include_str!("../tokens/fr.json"),
        LC::He => include_str!("../tokens/he.json"),
        LC::Id => include_str!("../tokens/id.json"),
        LC::It => include_str!("../tokens/it.json"),
        LC::Ja => include_str!("../tokens/ja.json"),
        LC::Nl => include_str!("../tokens/nl.json"),
        LC::No => include_str!("../tokens/no.json"),
        LC::Pl => include_str!("../tokens/pl.json"),
        LC::Pt => include_str!("../tokens/pt.json"),
        LC::Ro => include_str!("../tokens/ro.json"),
        LC::Ru => include_str!("../tokens/ru.json"),
        LC::Sv => include_str!("../tokens/sv.json")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_tokens() {
        let lc_tokens = tokens(vec![String::from("de"), String::from("en")]);
        assert_eq!(lc_tokens.len(), 2);
        assert!(lc_tokens.contains_key(&LC::De));
        assert!(lc_tokens.contains_key(&LC::En));

        let empty_lc = tokens(Vec::new());
        let every_lc = get_tokens(all_lcs());
        assert_eq!(empty_lc.len(), every_lc.len());
    }

    #[test]
    #[should_panic(expected = "LanguageCodeNotSupported")]
    fn fail_tokens() {
        tokens(vec![String::from("zz")]);
    }

    #[test]
    fn test_all_lcs() {
        let lcs = all_lcs();
        let file_system_lcs = to_lc(read_files()).unwrap();
        assert_eq!(lcs.len(), file_system_lcs.len());
    }

    #[test]
    fn test_to_lc() {
        assert_eq!(to_lc(vec![String::from("de")]).unwrap(), vec![LC::De]);
        assert_eq!(to_lc(vec![String::from("de"), String::from("en")]).unwrap(), vec![LC::De, LC::En]);
    }

    #[test]
    #[should_panic(expected = "LanguageCodeNotSupported")]
    fn fail_to_lc() {
        to_lc(vec![String::from("zz")]).unwrap();
    }

    #[test]
    fn test_get_tokens() {
        let lc_tokens = get_tokens(vec![LC::De, LC::En]);
        assert_eq!(lc_tokens.len(), 2);
        assert!(lc_tokens.contains_key(&LC::De));
        assert!(lc_tokens.contains_key(&LC::En));
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
        let token_map = tokens(Vec::new());

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
            let split = file_name.split(".");
            let file_components: Vec<String> = split.map(|file_name| {
                String::from(file_name)
            }).collect();
            if file_components[1] == String::from("json") {
                lcs.push(file_components[0].clone());
            }
        }
        lcs
    }
}
