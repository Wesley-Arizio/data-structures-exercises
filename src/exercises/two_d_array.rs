pub fn hourglass_sum(arr: &[Vec<i32>]) -> i32 {
    let mut greatest_sum = i32::MIN;
    for r in 0..4 {
        for c in 0..4 {
            let sum = arr[r][c]
                + arr[r][c + 1]
                + arr[r][c + 2]
                + arr[r + 1][c + 1]
                + arr[r + 2][c]
                + arr[r + 2][c + 1]
                + arr[r + 2][c + 2];

            if sum > greatest_sum {
                greatest_sum = sum;
            }
        }
    }

    greatest_sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_two_d_array() {
        let args = vec![
            vec![-9, -9, -9, 1, 1, 1],
            vec![0, -9, 0, 4, 3, 2],
            vec![-9, -9, -9, 1, 2, 3],
            vec![0, 0, 8, 6, 6, 0],
            vec![0, 0, 0, -2, 0, 0],
            vec![0, 0, 1, 2, 4, 0],
        ];

        assert_eq!(hourglass_sum(&args), 28);
    }
}
