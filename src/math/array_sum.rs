pub fn simple_array_sum(ar: &[i32]) -> i32 {
    let sum: i32 = ar.iter().sum();
    sum
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn basic_big_sum() {
        assert_eq!(simple_array_sum(&[2, 2]), 4);
        assert_eq!(simple_array_sum(&[1, 2, 3]), 6);
    }
}
