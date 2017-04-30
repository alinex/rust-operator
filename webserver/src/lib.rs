use std::error::Error;

pub fn run() -> Result<(), Box<Error>> {
    // Start the real application work...
    println!("Done.");
    let num = 10;
    println!("Hello, world! {} plus one is {}!", num, add_one(num));

    Ok(())
}

pub fn add_one(x: i32) -> i32 {
    x + 1
}
