use std::ops::AddAssign;

// Should validate if the sequence array appears in the main array in the same order
pub fn is_valid_subsequence(arr: Vec<i32>, sequence: Vec<i32>) -> bool {
    let mut sequence_cursor = 0;

    for i in 0..arr.len() {
        if sequence_cursor == sequence.len() {
            break;
        }

        if arr[i] == sequence[sequence_cursor] {
            sequence_cursor.add_assign(1);
        }
    }

    sequence_cursor == sequence.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn validate_subsequence_success() {
        let arr = [5, 1, 22, 25, 6, -1, 8, 10].to_vec();
        let sequence = [1, 6, -1, 10].to_vec();
        assert!(is_valid_subsequence(arr, sequence));

        let arr = [5, 1, 22, 25, 6, -1, 8, 10].to_vec();
        let sequence = [5, 1, 22, 25, 6, -1, 8, 10].to_vec();
        assert!(is_valid_subsequence(arr, sequence));

        let arr = [5, 1, 22, 25, 6, -1, 8, 10].to_vec();
        let sequence = [5, 1, 22, 25, 6, -1, 8, 10].to_vec();
        assert!(is_valid_subsequence(arr, sequence));

        let arr = [5, 1, 22, 25, 6, -1, 8, 10].to_vec();
        let sequence = [22, 25, 6].to_vec();
        assert!(is_valid_subsequence(arr, sequence));

        let arr = [5, 1, 22, 25, 6, -1, 8, 10].to_vec();
        let sequence = [1, 6, -1, 5].to_vec();
        assert!(!is_valid_subsequence(arr, sequence));

        let arr = [5, 1, 22, 25, 6, -1, 8, 10].to_vec();
        let sequence = [1, 6, -1, 10, 11, 11, 11, 11].to_vec();
        assert!(!is_valid_subsequence(arr, sequence));
    }
}