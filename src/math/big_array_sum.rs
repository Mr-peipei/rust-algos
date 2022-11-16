pub fn big_sum(ar: &[i64]) -> i64 {
    let sum: i64 = ar.iter().sum();
    sum
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn basic_big_sum() {
        assert_eq!(big_sum(&[2, 2]), 4);
        assert_eq!(big_sum(&[1, 2, 3]), 6);
    }
}
