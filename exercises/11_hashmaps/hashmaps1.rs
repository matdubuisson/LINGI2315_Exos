// A basket of fruits in the form of a hash map needs to be defined. The key
// represents the name of the fruit and the value represents how many of that
// particular fruit is in the basket. You have to put at least 3 different
// types of fruits (e.g. apple, banana, mango) in the basket and the total count
// of all the fruits should be at least 5.

use std::collections::HashMap;

const FRUITS: [&str; 4] = [
    "banana",
    "pear",
    "strawberry",
    "apple"
];

/*
Add this in Cargo.toml :
[dependencies]
rand = "0.8.3"
*/
use rand::Rng;

fn fruit_basket() -> HashMap<String, u32> {
    // TODO: Declare the hash map.
    // let mut basket =

    // Two bananas are already given for you :)
    let mut basket: HashMap<String, u32> = HashMap::new();
    let mut count: u32 = 0;
    let mut rng = rand::thread_rng();

    while basket.keys().len() < 3 || count < 5 {
        let fruit_choice: usize = rng.gen::<usize>() % FRUITS.len();
        let fruit = FRUITS.get(fruit_choice).unwrap();
        let value = basket.entry((*fruit).to_string()).or_insert(0);
        *value += 1;
        count += 1;
    }
    //basket.insert(String::from("banana"), 2);

    // TODO: Put more fruits in your basket.

    basket
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
