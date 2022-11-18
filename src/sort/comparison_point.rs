/*
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY a
 *  2. INTEGER_ARRAY b
 */

pub fn compare_triplets(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut vec = vec![0, 0];
    for i in 0..3 {
        if a[i] > b[i] {
            vec[0] += 1
        }
        if a[i] < b[i] {
            vec[1] += 1
        }
    }
    vec
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn basic_big_sum() {
        let a = [1, 2, 3];
        let b = [1, 2, 3];
        assert_eq!(compare_triplets(&a, &b), [0, 0]);
        let c = [1, 2, 3];
        let d = [1, 3, 3];
        assert_eq!(compare_triplets(&c, &d), [0, 1]);
        let e = [1, 2, 4];
        let f = [1, 2, 3];
        assert_eq!(compare_triplets(&e, &f), [1, 0]);
        let g = [2, 2, 3];
        let h = [1, 2, 4];
        assert_eq!(compare_triplets(&g, &h), [1, 1]);
    }
}
