const SPEED: u32 = 340;
static TEST: &str = "This is a beautiful message";

fn main () {
    let mut something: [u32; 10] = [1, 17, 3872, 38, 192, 328, 19, 19, 4628, 14];

    let mut i: usize = 0;

    while i < 10 {
        something[i] *= SPEED;
        print!("{} ", something[i]);
        i += 1;
    }

    println!();

    println!("{TEST}");
}

// https://stackoverflow.com/questions/52751597/what-is-the-difference-between-a-constant-and-a-static-variable-and-which-should

// const:

//     Have no fixed address in memory
//     They’re inlined to each place which uses them, this means they are put directly into the binary on the places which use them.
//     Usually faster runtime but bigger executable file because it doesn't have to look up an address like static

// static:

//     Have a fixed address in memory
//     Their value is loaded from this fixed address each place which uses them.
//     Usually slower runtime because we need to perform the extra instruction of loading the data from the fixed address. However this could result in a smaller executable file (only when it is used frequently) because it doesn't have to have multiple copies of the value baked into the executable.
