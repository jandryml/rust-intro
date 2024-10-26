
// can be declared in global cope
// only set by constant expression known at compile time
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

// let y = 5; // not possible

fn main() {
    println!("-- BASIC VARIABLES HORSING AROUND --");

    let x = 5;
    let mut y = 5;
    println!("The values are: {x}, {y}");
    y = 6;
    // x = 6; - not possible
    println!("The values are: {x}, {y}");

    println!("Seconds in three hours: {THREE_HOURS_IN_SECONDS}");

    shadowing();
}

fn shadowing() {
    println!();
    println!("-- SHADOWING --");

    let x = 5;
    println!("First innit: {x}");

    let x = x + 5;
    println!("Shadowed: {x}");

    {
        let x = x + 5;
        println!("Shadowed in inner scope: {x}");
    }

    println!("Former value from this scope is present: {x}");
}