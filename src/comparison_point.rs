/*
 * Complete the 'compareTriplets' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY a
 *  2. INTEGER_ARRAY b
 */

pub fn compareTriplets(a: &[i32], b: &[i32]) -> Vec<i32> {
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
