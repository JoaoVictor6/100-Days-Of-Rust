use rand;

const SKEWERS_FILLING_LENGTH: i8 = 8;
pub fn generate() -> String {
    let mut result = String::new();
    
    for _ in 1..=SKEWERS_FILLING_LENGTH {
        if rand::random() {
            result += "o";
            continue;
        }
        result += "x"
    }
    return "--".to_string() + result.as_str() + "--";
}
