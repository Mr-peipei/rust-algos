/*
 * Complete the 'aVeryBigSum' function below.
 *
 * The function is expected to return a LONG_INTEGER.
 * The function accepts LONG_INTEGER_ARRAY ar as parameter.
 */

pub fn aVeryBigSum(ar: &[i64]) -> i64 {
    let sum: i64 = ar.iter().sum();
    sum
}
