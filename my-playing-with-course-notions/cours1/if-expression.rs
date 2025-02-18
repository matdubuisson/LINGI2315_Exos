fn main () {
    let x = 21;
    
    if x < 20 {
        println!("Hey boy");
    } else if x < 60 {
        println!("Hey man");
    } else {
        println!("Hey papy");
    }

    let answer = if x <= 40 { "espoir" } else if x <= 60 { "senior" } else { "master" };
    println!("You are {}", answer);
}