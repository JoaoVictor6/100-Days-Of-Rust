mod generate_non_vegetarian_skewers;
fn main() {
    let result = barbecue_skewers([1,2]);
    println!("{}", result.join("\n"));
}

fn barbecue_skewers (orders: [i8; 2]) -> Vec<String> {
    let [ vegetarian_skewers, non_vegetarian_skewers ] = orders;
    let mut result: Vec<String> = Vec::new();
    let mut vegetarian_remaining = vegetarian_skewers;
    let mut non_vegetarian_remaining = non_vegetarian_skewers;
    
    while vegetarian_remaining + non_vegetarian_remaining != 0 {
        if vegetarian_remaining != 0 {
            result.push("--oooo-ooo--".to_string());
            vegetarian_remaining = vegetarian_remaining - 1;
        } else {
            result.push(generate_non_vegetarian_skewers::generate());
            non_vegetarian_remaining = non_vegetarian_remaining - 1;
        }
    }

    return result;
}

#[cfg(test)]
mod test {
    #[test]
    fn should_pass () {
        let prompt = [1, 4];
    }
}
