fn main () {
    let mut a: [i8; 10] = [42; 10]; // Init an array of 10 int8 with value 42
    a[5] = 0; // Set sixth value to 0
    println!("a: {:?}", a);

    let t: (i8, bool, i32) = (7, true, 32000);
    println!("t: {:?}", t);
    println!("1st: {}, 2nd: {}, 3rd: {}", t.0, t.1, t.2);
    // Not allowed for tuples => println!("1st: {}, 2nd: {}, 3rd: {}", t[0], t[1], t[2]);
}