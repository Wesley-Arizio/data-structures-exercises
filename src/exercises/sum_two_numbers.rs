use std::ops::{Add, AddAssign, Sub, SubAssign};

pub fn sum_two_numbers(array: Vec<i32>, target_sum: i32) -> Vec<i32> {
    if array.len() <= 1 {
        return [].to_vec();
    };

    let mut array = array;
    array.sort();
    let mut left = 0;
    let mut right = array.len().sub(1);

    while left < right {
        let sum = array[left].add(array[right]);

        if sum == target_sum {
            return [array[left], array[right]].to_vec();
        } else if sum < target_sum {
            left.add_assign(1);
        } else if sum > target_sum {
            right.sub_assign(1);
        } else {
            break;
        }
    }

    [].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_two_numbers_success() {
        let array = [3, 5, -4, 8, 11, 1, -1, 6].to_vec();
        let target_sum = 10;
        assert_eq!(sum_two_numbers(array, target_sum), [-1, 11].to_vec());

        let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 15].to_vec();
        let target_sum = 18;
        assert_eq!(sum_two_numbers(array, target_sum), [3, 15].to_vec());

        let array = [14].to_vec();
        let target_sum = 15;
        assert_eq!(sum_two_numbers(array, target_sum), [].to_vec());

        let array = [1, 1].to_vec();
        let target_sum = 15;
        assert_eq!(sum_two_numbers(array, target_sum), [].to_vec());

        let array = [4, 6].to_vec();
        let target_sum = 10;
        assert_eq!(sum_two_numbers(array, target_sum), [4, 6].to_vec());

        let array = [4, 6, 1].to_vec();
        let target_sum = 5;
        assert_eq!(sum_two_numbers(array, target_sum), [1, 4].to_vec());

        let array = [4, 6, 1, -3].to_vec();
        let target_sum = 3;
        assert_eq!(sum_two_numbers(array, target_sum), [-3, 6].to_vec());

        let array = [1, 2, 3, 4, 5, 6, 7, 8, 9].to_vec();
        let target_sum = 17;
        assert_eq!(sum_two_numbers(array, target_sum), [8, 9].to_vec());

        let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 15].to_vec();
        let target_sum = 18;
        assert_eq!(sum_two_numbers(array, target_sum), [3, 15].to_vec());

        let array = [-7, -5, -3, -1, 0, 1, 3, 5, 7].to_vec();
        let target_sum = -5;
        assert_eq!(sum_two_numbers(array, target_sum), [-5, 0].to_vec());

        let array = [-21, 301, 12, 4, 65, 56, 210, 356, 9, -47].to_vec();
        let target_sum = 163;
        assert_eq!(sum_two_numbers(array, target_sum), [-47, 210].to_vec());
    }
}
