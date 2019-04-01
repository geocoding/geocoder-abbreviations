use serde::Deserialize;
use serde_json;

#[derive(Debug)]
enum CC {
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

fn main() -> Result<(), Error> {
    let tokens = match_tokens(CC::De).unwrap();
    let parsed_tokens : Vec<Token> = serde_json::from_str(&tokens);
    println!("{:#?}", parsed_tokens[0]);
    Ok(())
}

fn match_tokens(cc: CC) -> Result<String, Error> {
    match cc {
        CC::De => {
            Ok(include_str!("../tokens/de.json").to_string())
        }
        CC::En => {
            Ok(include_str!("../tokens/en.json").to_string())
        },
        CC::Es => {
            Ok(include_str!("../tokens/es.json").to_string())
        },
        CC::Fi => {
            Ok(include_str!("../tokens/fi.json").to_string())
        },
        _ => Err(Error::CountryCodeNotSupported)
    }
}
