use std::convert::{TryFrom, TryInto};

pub fn dynamic_array(n: i32, queries: &[Vec<i32>]) -> Vec<i32> {
    let mut storage: Vec<Vec<i32>> = vec![vec![]; usize::try_from(n).unwrap()];
    let mut last_answer = 0;
    let mut result: Vec<i32> = Vec::new();

    for v in queries {
        let query = v[0];
        let x = v[1];
        let y = v[2];
        let i = usize::try_from((x ^ last_answer) % n).expect("Could not conver i to usize");

        match query {
            1 => {
                storage[i].push(y);
            }
            2 => {
                let size: i32 = storage[i]
                    .len()
                    .try_into()
                    .expect("Could not convert array size to i32");
                let j: usize =
                    usize::try_from(y % size).expect("Could not calculate query y value");
                last_answer = storage[i][j];
                result.push(last_answer);
            }
            _ => panic!("unexpected first arg"),
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_dynamic_array() {
        let queries = vec![
            vec![1, 0, 5],
            vec![1, 1, 7],
            vec![1, 0, 3],
            vec![2, 1, 0],
            vec![2, 1, 1],
        ];

        assert_eq!(dynamic_array(2, &queries), vec![7, 3])
    }
}
