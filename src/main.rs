use anyhow::{Context, Result};

const API_ENDPOINT: &str = "https://api.dictionaryapi.dev/api/v2/entries/en_US";

#[derive(serde::Deserialize)]
struct ParsedResponse {
    meanings: Vec<Meanings>,
}

#[derive(serde::Deserialize)]
struct Meanings {
    definitions: Vec<Definition>,
}

#[derive(serde::Deserialize)]
struct Definition {
    definition: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let word = std::env::args().nth(1).context("Please enter a word.")?;

    let definition = get_word_definition(word).await?;
    for meaning in definition.meanings {
        for definition in meaning.definitions {
            println!("{}", definition.definition)
        }
    }

    Ok(())
}

fn format_resource_uri(word: String) -> String {
    format!("{}/{}", API_ENDPOINT, word)
}

async fn get_word_definition(word: String) -> Result<ParsedResponse> {
    let url = format_resource_uri(word);
    let response = reqwest::get(url).await?;

    let parsed_response: Vec<ParsedResponse> = response
        .json()
        .await
        .context("Please enter a valid word.")?;
    parsed_response
        .into_iter()
        .nth(0)
        .context("Could not find first element.")
}
