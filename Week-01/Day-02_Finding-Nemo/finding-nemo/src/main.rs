fn main() {
    let value = "I Nemo am".to_owned();
    println!("{}", find_nemo(value));
}

fn find_nemo(sentence: String) -> String {
    let mut nemo_position:i8 = 4;

    let parts = sentence.split(" ");
    let mut index: i8 = 1;
    for part in parts {
        if part == "Nemo" {
            nemo_position = index
        }
        index = index + 1;
    }
    

    let result_phrase = "I found Nemo at ".to_owned() + &nemo_position.to_string() + "!";
    return result_phrase
}

#[cfg(test)]
mod tests {
    use crate::find_nemo;

    #[test]
    fn should_pass_first_sentence() {
        let sentence = "I am finding Nemo !";
        let expected_sentence = "I found Nemo at 4!";
        let result = find_nemo(sentence.to_string());
        assert_eq!(result, expected_sentence);
    }
    #[test]
    fn should_pass_second_sentence() {
        let sentence = "Nemo is me";
        let expected_sentence = "I found Nemo at 1!";
        let result = find_nemo(sentence.to_string());
        assert_eq!(result, expected_sentence);
    }
    #[test]
    fn should_pass_third_sentence() {
        let sentence = "I Nemo am";
        let expected_sentence = "I found Nemo at 2!";
        let result = find_nemo(sentence.to_string());
        assert_eq!(result, expected_sentence);
    }
}
