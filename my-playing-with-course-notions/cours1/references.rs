fn main () {
    let mut x: i32 = 10;
    let ref_x: &mut i32 = &mut x; // Mutable reference, only one allowed
    *ref_x = 20;
    println!("x: {x}");
}