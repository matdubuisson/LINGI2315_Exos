fn main () {
    let s1: &str = "World";
    println!("s1: {s1}");

    // let s2: String = String::from("Hello "); => Produce error
    let mut s2: String = String::from("Hello ");
    println!("s2: {s2}");

    s2.push_str(s1);
    println!("s2: {s2}");

    let s3: &str = &s2[6..];
    println!("s3: {s3}");
}


// &str an immutable reference to a string slice.
// String a mutable string buffer.
