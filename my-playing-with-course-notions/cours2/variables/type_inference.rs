fn print_u32 (x: u32) {
    println!("u32: {x}");
}

fn print_i8 (x: i8) {
    println!("i8: {x}");
}

fn main () {
    let x = 10;
    let y = 20;

    print_u32(x);
    print_i8(y);
    // print_i8(x); => Produce error as x is automatically typed to u32
}