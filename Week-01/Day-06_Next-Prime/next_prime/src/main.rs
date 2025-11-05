fn next_prime(n: u32) -> u32 {
    fn is_prime(num: u32) -> bool {
        if num < 2 {
            return false;
        }

        for i in 2..=((num as f64).sqrt() as u32) {
            if num % i == 0 {
                return false;
            }
        }
        true    
    }
    let mut candidate = n;
    while !is_prime(candidate) {
        candidate += 1;
    }
    candidate
}

fn main() {
    println!("{}", next_prime(12)); // Output: 13
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_prime() {
        assert_eq!(next_prime(12), 13);
        assert_eq!(next_prime(24), 29);
        assert_eq!(next_prime(11), 11);
    }
}
