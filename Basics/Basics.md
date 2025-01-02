# Basics of Rust: A Comprehensive Guide

This guide covers the essential concepts of Rust programming with examples and results for every beginner to learn and reference. 🚀

---

## **1. Setup and Environment**

### Install Rust:
Run the following command:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rust-lang.org | sh
```

### Verify Installation:
```bash
rustc --version
```

### Create a Project:
```bash
cargo new my_project
cd my_project
```

---

## **2. Hello, World!** 👋

```rust
fn main() {
    println!("Hello, World!");
}
```
### Run:
```bash
cargo run
```
### Output:
```
Hello, World!
```

---

## **3. Variables and Mutability** 🔄

### Example:
```rust
fn main() {
    let x = 5; // immutable
    let mut y = 10; // mutable
    y += 5;
    println!("x: {}, y: {}", x, y);
}
```
### Output:
```
x: 5, y: 15
```

---

## **4. Data Types** 📊

### Example:
```rust
fn main() {
    let integer: i32 = 42;
    let floating: f64 = 3.14;
    let boolean: bool = true;
    let character: char = 'R';
    println!("{} {} {} {}", integer, floating, boolean, character);
}
```
### Output:
```
42 3.14 true R
```

---

## **5. Control Flow** 🔁

### If-Else:
```rust
fn main() {
    let number = 7;
    if number < 10 {
        println!("Single digit");
    } else {
        println!("Double digit");
    }
}
```
### Output:
```
Single digit
```

### Loops:
```rust
fn main() {
    for i in 1..=5 {
        println!("Number: {}", i);
    }
}
```
### Output:
```
Number: 1
Number: 2
Number: 3
Number: 4
Number: 5
```

---

## **6. Functions** 🛠️

### Example:
```rust
fn main() {
    greet("Rustacean");
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}
```
### Output:
```
Hello, Rustacean!
```

---

## **7. Ownership and Borrowing** 📜

### Ownership:
```rust
fn main() {
    let s = String::from("Hello");
    takes_ownership(s);
    // println!("{}", s); // Error: s is no longer valid
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}
```

### Borrowing:
```rust
fn main() {
    let s = String::from("Hello");
    let len = calculate_length(&s);
    println!("The length of '{}' is {}.", s, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```
### Output:
```
The length of 'Hello' is 5.
```

---

## **8. Structs and Enums** 🏗️

### Struct:
```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    println!("Rectangle: {} x {}", rect.width, rect.height);
}
```
### Output:
```
Rectangle: 30 x 50
```

### Enum:
```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let dir = Direction::Up;
    match dir {
        Direction::Up => println!("Going up!"),
        Direction::Down => println!("Going down!"),
        Direction::Left => println!("Going left!"),
        Direction::Right => println!("Going right!"),
    }
}
```
### Output:
```
Going up!
```

---

## **9. Collections** 📚

### Vectors:
```rust
fn main() {
    let mut v = vec![1, 2, 3];
    v.push(4);
    println!("{:?}", v);
}
```
### Output:
```
[1, 2, 3, 4]
```

### HashMap:
```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Red", 20);
    println!("{:?}", scores);
}
```
### Output:
```
{"Blue": 10, "Red": 20}
```

---

## **10. Error Handling** ⚠️

### Example:
```rust
fn main() {
    let result: Result<i32, &str> = divide(10, 2);
    match result {
        Ok(v) => println!("Result: {}", v),
        Err(e) => println!("Error: {}", e),
    }
}

fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("Division by zero")
    } else {
        Ok(a / b)
    }
}
```
### Output:
```
Result: 5
```

---

## **Resources to Learn More** 📖

- **[Rust Book](https://doc.rust-lang.org/book/)**
- **[Rust by Example](https://doc.rust-lang.org/rust-by-example/)**
- **Practice:** Build small projects like a calculator or a to-do app.

---

Happy coding with Rust! 🎉🦀
