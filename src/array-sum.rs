/*
 * Complete the 'simpleArraySum' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY ar as parameter.
 */

pub fn simpleArraySum(ar: &[i32]) -> i32 {
    let sum: i32 = ar.iter().sum();
    sum
}
