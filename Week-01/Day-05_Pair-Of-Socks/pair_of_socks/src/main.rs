use std::collections::HashMap;

fn main() {
    let result = socks_pair("CABBACCC");
    println!("SockPairs(\"CABBACCC\") -> {}", result);
}

const MINIMUM_SOCKS_QUANTITY: i8 = 1;
fn socks_pair (socks_sequence: &str) -> i8 {
    let mut pairs_quantity = 0;
    let socks_units = socks_sequence.split("");
    let mut socks: HashMap<&str, i8> = HashMap::new();

    for socks_unit in socks_units {
        if socks_unit.is_empty() {
            continue;
        }
        let has_this_socks = socks.contains_key(socks_unit);
        if has_this_socks {
            let saved_socks_quantity = socks.get(socks_unit).copied().unwrap();

            socks.insert(socks_unit, saved_socks_quantity + 1);
            continue;
        }
        socks.insert(socks_unit, 1);
    }
    for (_, quantity) in socks.iter() {
        if quantity.to_owned() > MINIMUM_SOCKS_QUANTITY {
            if quantity % 2 == 0 {
                pairs_quantity += quantity / 2;
            }
            if quantity % 2 > 1 {
                pairs_quantity += quantity - 1 / 2;
            }
        }
    }

    return pairs_quantity;
}


# [cfg(test)]
mod test {
    use crate::socks_pair;

    # [test]
    fn should_pass () {
        let test_cases: [(&str, i8); 3] = [
            ("AA", 1),
            ("ABABC", 2),
            ("CABBACCC", 4)
        ];

        for (arg, expected_result) in test_cases {
            let result = socks_pair(arg);
            assert_eq!(result, expected_result);
        }
    }
}