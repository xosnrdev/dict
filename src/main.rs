const API_ENDPOINT: &str = "https://api.dictionaryapi.dev/api/v2/entries/en_US";

#[tokio::main]
async fn main() {
    let word = std::env::args().nth(1);

    if let Some(word) = word {
        println!("{}", get_word_definition(word).await)
    }
}

fn resource_uri(word: String) -> String {
    format!("{}/{}", API_ENDPOINT, word)
}

async fn get_word_definition(word: String) -> String {
    let url = resource_uri(word);
    let response = reqwest::get(url).await.expect("Could not fetch resource.");

    response.text().await.expect("Could not read the response.")
}
