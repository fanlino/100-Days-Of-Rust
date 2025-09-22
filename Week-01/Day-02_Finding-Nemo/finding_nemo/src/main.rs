fn find_nemo(sentence: &str) -> String {
    let words: Vec<&str> = sentence.split_whitespace().collect();
    for (i, word) in words.iter().enumerate() {
        if *word == "Nemo" {
            return format!("I found Nemo at {}!", i + 1);
        }
    }
    "I couldn't find Nemo :(".to_string()
}
fn main() {
    let sentences = ["I am finding Nemo !", "Nemo is me", "I Nemo am", "I am not here"];
    for sentence in sentences {
        let result = find_nemo(sentence);
        println!("{}", result);
    }
}
