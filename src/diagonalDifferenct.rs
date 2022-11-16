/*
 * Complete the 'diagonalDifference' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts 2D_INTEGER_ARRAY arr as parameter.
 * Given a square matrix, calculate the absolute difference between the sums of its diagonals.
 * For example, the square matrix  is shown below:
 * 1 2 3
 * 4 5 6
 * 9 8 9
 *  The left-to-right diagonal = . The right to left diagonal = . Their absolute difference is .
 */
pub fn diagonalDifference(arr: &[Vec<i32>]) -> i32 {
    let mut left = 0;
    let mut right = 0;
    for i in 0..arr.len() {
        left += arr[i][i];
        right += arr[i][arr.len() - i - 1];
    }
    let mut sum: i32 = left - right;
    if sum < 0 {
        sum *= -1;
    }
    sum
}
