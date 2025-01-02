# Advanced Rust Concepts with Examples and Results

## **1. Ownership, Borrowing, and Lifetimes**

### **Example: Ownership and Borrowing**
```rust
fn main() {
    let s = String::from("Hello, Rust!");
    borrow_string(&s);
    println!("Original string: {}", s);
}

fn borrow_string(s: &String) {
    println!("Borrowed string: {}", s);
}
```
#### **Result:**
```
Borrowed string: Hello, Rust!
Original string: Hello, Rust!
```

### **Example: Lifetimes**
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("Rustacean");
    let string2 = "Programming";

    let result = longest(&string1, string2);
    println!("The longest string is: {}", result);
}
```
#### **Result:**
```
The longest string is: Programming
```

---

## **2. Traits and Generics**

### **Example: Implementing Traits**
```rust
trait Greet {
    fn greet(&self);
}

struct Person {
    name: String,
}

impl Greet for Person {
    fn greet(&self) {
        println!("Hello, {}!", self.name);
    }
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
    };
    person.greet();
}
```
#### **Result:**
```
Hello, Alice!
```

### **Example: Generics**
```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    println!("The largest number is {}", largest(&numbers));
}
```
#### **Result:**
```
The largest number is 100
```

---

## **3. Concurrency**

### **Example: Threads**
```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Spawned thread: {}", i);
        }
    });

    for i in 1..5 {
        println!("Main thread: {}", i);
    }

    handle.join().unwrap();
}
```
#### **Result:**
```
Main thread: 1
Main thread: 2
Main thread: 3
Main thread: 4
Spawned thread: 1
Spawned thread: 2
... (continues)
```

### **Example: Channels**
```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Hello, World!");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Received: {}", received);
}
```
#### **Result:**
```
Received: Hello, World!
```

---

## **4. Asynchronous Programming**

### **Example: Async/Await**
```rust
use tokio::time::{sleep, Duration};

async fn do_something() {
    println!("Doing something...");
    sleep(Duration::from_secs(2)).await;
    println!("Done!");
}

#[tokio::main]
async fn main() {
    do_something().await;
}
```
#### **Result:**
```
Doing something...
Done!
```

---

## **5. Unsafe Rust**

### **Example: Dereferencing a Raw Pointer**
```rust
fn main() {
    let num = 5;
    let r = &num as *const i32;

    unsafe {
        println!("Dereferenced value: {}", *r);
    }
}
```
#### **Result:**
```
Dereferenced value: 5
```

---

## **6. Advanced Macros**

### **Example: Declarative Macro**
```rust
macro_rules! repeat {
    ($val:expr, $times:expr) => {
        for _ in 0..$times {
            println!("{}", $val);
        }
    };
}

fn main() {
    repeat!("Hello", 3);
}
```
#### **Result:**
```
Hello
Hello
Hello
```

---

## **7. Real-World Application: Web Server**

### **Example: Using Warp for Web Development**
```rust
use warp::Filter;

#[tokio::main]
async fn main() {
    let route = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    warp::serve(route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
```
#### **Run:**
```bash
cargo run
```
#### **Access:**
`http://127.0.0.1:3030/hello/Rust`

#### **Result:**
```
Hello, Rust!
```

---

## **8. Further Learning Resources**
- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Advanced Rust Programming Tutorials](https://doc.rust-lang.org/nomicon/)
