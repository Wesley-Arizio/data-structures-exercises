pub fn get_total(a: &[i32], b: &[i32]) -> i32 {
    let max = a.iter().max().cloned().unwrap_or_default();
    let min = b.iter().min().cloned().unwrap_or_default();

    let mut result: i32 = 0;

    for i in max..(min + 1) {
        let mut is_factor_multiple = true;

        for v in a {
            if i % v != 0 {
                is_factor_multiple = false;
                break;
            }
        }

        for v in b {
            if v % i != 0 {
                is_factor_multiple = false;
                break;
            }
        }

        if is_factor_multiple {
            result += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_total() {
        let a = [2, 4];
        let b = [16, 32, 96];

        assert_eq!(get_total(&a, &b), 3);
    }
}
