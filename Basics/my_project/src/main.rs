fn main() {
    println!("Hello, world!");
    println!("--------------");

    let x = 5;
    let mut y = 10;
    y += 5;
    println!("x: {}, y: {}", x, y);
    println!("--------------");

    let number = 7;
    if number < 10 {
        println!("Single digit");
    } else {
        println!("Double digit");
    }
    println!("--------------");

    for i in 1..=5 {
        println!("Number: {}", i);
    }
    println!("--------------");

    greet("Rustacean");
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}