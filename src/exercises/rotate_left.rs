pub fn rotate_left(d: i32, arr: &[i32]) -> Vec<i32> {
    let mut result = arr.clone().to_vec();

    for _i in 0..d {
        let first_element = result.remove(0);
        result.push(first_element);
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rotate_left() {
        let d = 4;
        let v = vec![1, 2, 3, 4, 5];
        let expected = vec![5, 1, 2, 3, 4];
        assert_eq!(rotate_left(d, &v), expected);
    }
}
