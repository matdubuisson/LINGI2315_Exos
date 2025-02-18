fn main() {
    let a = 10;
    println!("before: {a}");

    {
        // Access to global a

        let a = "hello";
        println!("inner scope: {a}");
        // Access to nearest local a

        let a = true; // Create new var reusing identifier a
        println!("shadowed in inner scope: {a}");
        // Access to nearest local a (second one)
    }

    // Access to global a
    println!("after: {a}");
}