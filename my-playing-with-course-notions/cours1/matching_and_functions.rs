fn main () {
    fun_with(2 * 3 * 7);
}

fn is_multiple(k: u32, m:u32) -> bool {
    return k % m == 0;
}

fn check(k: u32) -> () {  // No return value means returning the unit type `()`
    match (is_multiple(k, 2), is_multiple(k, 3), is_multiple(k, 7)) {
        (true, true, true) => println!("Full"),
        (true, false, true) => println!("Not 3"),
        _ => println!("Something else"),
    }
}

fn fun_with(n: u32) { // `-> ()` is normally omitted
    for i in 1..=n {
        print!("{i}) ");
        check(i);
    }
}