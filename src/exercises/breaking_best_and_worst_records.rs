pub fn breaking_records(scores: &[i32]) -> Vec<i32> {
    let mut best_scores_counter = 0;
    let mut worst_scores_counter = 0;

    let mut current_best_score = scores.first().cloned().unwrap_or_default();
    let mut current_worst_score = scores.first().cloned().unwrap_or_default();

    for v in scores {
        if v > &current_best_score {
            best_scores_counter += 1;
            current_best_score = *v;
        }

        if v < &current_worst_score {
            worst_scores_counter += 1;
            current_worst_score = *v;
        }
    }

    vec![best_scores_counter, worst_scores_counter]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_breaking_records() {
        let scores = vec![10, 5, 20, 20, 4, 5, 2, 25, 1];

        assert_eq!(breaking_records(&scores), [2, 4]);
    }
}
