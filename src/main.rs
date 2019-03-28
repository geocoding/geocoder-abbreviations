use serde::{Deserialize};
use serde_json::Result;

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

fn main() -> Result<()> {
    get_tokens()?;
    Ok(())
}

fn get_tokens() -> Result<Vec<Token>> {
    let my_str = include_str!("../tokens/de.json");
    serde_json::from_str(my_str)
}
