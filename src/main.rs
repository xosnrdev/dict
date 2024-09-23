const API_ENDPOINT: &str = "https://api.dictionaryapi.dev/api/v2/entries/en_US";

fn main() {
    let word = std::env::args().nth(1);

    if let Some(word) = word {
        println!("{}", resource_uri(word))
    }
}

fn resource_uri(word: String) -> String {
    format!("{}/{}", API_ENDPOINT, word)
}
