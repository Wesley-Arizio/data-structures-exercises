
// Should receive a sorted array and for each iteration, calculate its square and return its result
pub fn sorted_squared_array(array: Vec<i32>) -> Vec<i32> {
    let mut new_arr = Vec::new();

    for i in 0..array.len() {
        new_arr.push(array[i].pow(2));
    }

    new_arr.sort();

    new_arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorted_squared_array_success() {
        let arr = [1, 2, 3, 5, 6, 8, 9].to_vec();
        let expected = [1, 4, 9, 25, 36, 64, 81].to_vec();
        assert_eq!(sorted_squared_array(arr), expected);

        let arr = [-10, -5, 0, 5, 10].to_vec();
        let expected = [0, 25, 25, 100, 100].to_vec();
        assert_eq!(sorted_squared_array(arr), expected);
    }
}