fn count_skewers(sentences: Vec<&str>) -> [u32; 2] {
    let mut vegetarian_count = 0;
    let mut meat_count = 0;
    for (_i, sentence) in sentences.iter().enumerate() {
        if sentence.contains("x") {
            meat_count += 1;
        } else {
            vegetarian_count += 1;
        }
    }
    [vegetarian_count, meat_count]
}
fn main() {
    let sentences = [
        "--oooo-ooo--",
        "--xx--x--xx--",
        "--o---o--oo--",
        "--xx--x--ox--",
        "--xx--x--ox--",
    ];
    let result = count_skewers(sentences.to_vec());
    println!(
        "Vegetarian skewers: {}, Non-vegetarian skewers: {}",
        result[0], result[1]
    );
}
