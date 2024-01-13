fn main() {
    let prompt = progress_days([10, 11, 12, 9, 10].to_vec());
    println!("progressDays([10, 11, 12, 9, 10]) -> {}", prompt);
}

fn progress_days (progression_arr: Vec<i8>) -> i8 {
    let mut progression_quantity: i8 = 0;
    let mut old_progression_number: Option<i8> = None;
    for progression in progression_arr {
        match old_progression_number {
            Some(value) => {
                if progression > value {
                    progression_quantity = progression_quantity + 1;
                }
            }
            None => {},
        }
        let _ = old_progression_number.insert(progression);
    }

    return progression_quantity;
}

# [cfg(test)]
mod test {
    use crate::progress_days;

    # [test]
    fn should_pass() {
        let test_case: [(Vec<i8>, i8); 4] = [
            ([3,4,1,2].to_vec(), 2),
            ([10,11,12,9,10].to_vec(), 3),
            ([6,5,4,3,2,9].to_vec(), 1),
            ([9, 9].to_vec(), 0)
        ];

        for (input, expected_result) in test_case {
            let result = progress_days(input);

            assert_eq!(result, expected_result)
        }
    }
}