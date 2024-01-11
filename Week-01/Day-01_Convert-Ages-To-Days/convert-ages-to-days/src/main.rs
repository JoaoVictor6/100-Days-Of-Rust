use std::i16;

fn main() {
    println!("Hello, world!");
}

fn convert_ages_to_days (age: i16) -> i16 {
    return age*365
}

#[cfg(test)]
mod tests {
    use crate::convert_ages_to_days;

    #[test]
    fn receive_65() {
        let result = convert_ages_to_days(65);
        let expected_result: i16 = 23725;
        assert_eq!(result, expected_result);
    }
    #[test]
    fn receive_0() {
        let result = convert_ages_to_days(0);
        let expected_result: i16 = 0;
        assert_eq!(result, expected_result);
    }
    #[test]
    fn receive_20() {
        let result = convert_ages_to_days(20);
        let expected_result: i16 = 7300;
        assert_eq!(result, expected_result);
    }
}