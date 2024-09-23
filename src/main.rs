fn main() {
    let word = std::env::args().nth(1);

    if let Some(word) = word {
        println!("{}", word)
    }
}
