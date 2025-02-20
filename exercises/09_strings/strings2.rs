// TODO: Fix the compiler error in the `main` function without changing this function.
fn is_a_color_word(attempt: &str) -> bool {
    attempt == "red" || attempt == "green" || attempt == "blue"
}

fn main() {
    let word = String::from("green"); // Don't change this line.

    if is_a_color_word(word.as_str()) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}
