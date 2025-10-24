use std::collections::HashMap;

fn sock_pairs(socks: &str) -> i32 {
    let mut color_count = HashMap::new();
    for sock in socks.chars() {
        *color_count.entry(sock).or_insert(0) += 1;
    }
    let mut pairs = 0;
    for &count in color_count.values() {
        pairs += count / 2;
    }
    pairs
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sock_pairs() {
        assert_eq!(sock_pairs("AA"), 1);
        assert_eq!(sock_pairs("ABABC"), 2);
        assert_eq!(sock_pairs("CABBACCC"), 4);
    }
}
