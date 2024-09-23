const API_ENDPOINT: &str = "https://api.dictionaryapi.dev/api/v2/entries/en_US";

#[derive(serde::Deserialize, Debug)]
struct ParsedResponse {
    meanings: Vec<Meanings>,
}

#[derive(serde::Deserialize, Debug)]
struct Meanings {
    definitions: Vec<Definition>,
}

#[derive(serde::Deserialize, Debug)]
struct Definition {
    definition: String,
}

#[tokio::main]
async fn main() {
    let word = std::env::args().nth(1);

    if let Some(word) = word {
        let definition = get_word_definition(word).await;
        for meaning in definition.meanings {
            for definition in meaning.definitions {
                println!("{}", definition.definition)
            }
        }
    }
}

fn resource_uri(word: String) -> String {
    format!("{}/{}", API_ENDPOINT, word)
}

async fn get_word_definition(word: String) -> ParsedResponse {
    let url = resource_uri(word);
    let response = reqwest::get(url).await.expect("Could not fetch resource.");

    let parsed_response: Vec<ParsedResponse> =
        response.json().await.expect("Could not read the response.");
    parsed_response
        .into_iter()
        .nth(0)
        .expect("Could not find first element.")
}
