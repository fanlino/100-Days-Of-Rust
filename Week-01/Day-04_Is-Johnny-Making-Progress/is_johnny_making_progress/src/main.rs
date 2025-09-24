fn progress_days(progress: Vec<i32>) -> i32 {
    let mut count = 0;
    for i in 1..progress.len() {
        if progress[i] > progress[i - 1] {
            count += 1;
        }
    }
    count
}

fn main() {
    let progress = vec![1, 2, 3, 2, 5];
    let days = progress_days(progress);
    println!("Johnny made progress on {} days.", days);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_days() {
        assert_eq!(progress_days(vec![3, 4, 1, 2]), 2);
        assert_eq!(progress_days(vec![10, 11, 12, 9, 10]), 3);
        assert_eq!(progress_days(vec![6, 5, 4, 3, 2, 9]), 1);
        assert_eq!(progress_days(vec![9, 9]), 0);
    }
}
