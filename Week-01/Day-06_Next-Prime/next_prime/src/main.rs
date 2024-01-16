fn main() {
    let result = next_prime(12);
    println!("NextPrime(12) ➞ {}", result);
}

fn is_prime (number: i8) -> bool {
    let mut is_prime = true;
    let mut changed_number = number;
    if number < 2 {
        (false);
    }
    while changed_number > 2 {
        if changed_number == number {
            changed_number = changed_number - 1;
            continue;
        }
        if number % changed_number == 0 {
            is_prime = false;
            break;
        }
        changed_number = changed_number - 1;
    }
    
    return is_prime;
}

fn next_prime (number_input: i8) -> i8 {
    let mut changed_number = number_input;
    loop {
        if is_prime(changed_number) {
            return  changed_number;
        }
        changed_number = changed_number + 1;
    }
}

# [cfg(test)]
mod test {
    use crate::next_prime;

    # [test]
    fn should_pass () {
        let test_case: [(i8, i8); 3] = [
            (12, 13),
            (24, 29),
            (11, 11)
        ];

        for (input, expected_result) in test_case {
            let result = next_prime(input);

            assert_eq!(result, expected_result)
        }
    }
}