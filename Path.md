# 🚀 Learning Rust: Beginner to Advanced

## **📚 Phase 1: Basics of Rust**

### 1️⃣ **Setup**
- 🛠️ Install Rust using `rustup`:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rust-lang.org | sh
  ```
- ✅ Verify installation:
  ```bash
  rustc --version
  ```
- 📁 Set up a project:
  ```bash
  cargo new hello_rust
  cd hello_rust
  ```

### 2️⃣ **Hello, World!**
- Edit `src/main.rs`:
  ```rust
  fn main() {
      println!("Hello, World!");
  }
  ```
- ▶️ Run:
  ```bash
  cargo run
  ```

### 3️⃣ **Variables and Mutability**
- Example Code:
  ```rust
  fn main() {
      let x = 5; // immutable
      let mut y = 10; // mutable
      y += 5;
      println!("x: {}, y: {}", x, y);
  }
  ```
- 🖨️ Output: `x: 5, y: 15`

---

## **📖 Phase 2: Intermediate Concepts**

### 4️⃣ **Ownership and Borrowing**
- Example Code:
  ```rust
  fn main() {
      let s = String::from("Hello");
      takes_ownership(s); // s is moved
      // println!("{}", s); // ❌ Error: s is no longer valid
      let x = 5;
      makes_copy(x); // x is copied
      println!("{}", x); // Works fine
  }

  fn takes_ownership(some_string: String) {
      println!("{}", some_string);
  }

  fn makes_copy(some_integer: i32) {
      println!("{}", some_integer);
  }
  ```

### 5️⃣ **Lifetimes**
- Example Code:
  ```rust
  fn main() {
      let r;
      {
          let x = 5;
          r = &x; // ❌ Error: x does not live long enough
      }
      // println!("{}", r);
  }
  ```

---

## **🚀 Phase 3: Advanced Concepts**

### 6️⃣ **Concurrency**
- Example Code:
  ```rust
  use std::thread;

  fn main() {
      let handle = thread::spawn(|| {
          for i in 1..10 {
              println!("Hi number {} from the spawned thread!", i);
          }
      });

      for i in 1..5 {
          println!("Hi number {} from the main thread!", i);
      }

      handle.join().unwrap();
  }
  ```

### 7️⃣ **Building a Web Server**
#### 🔗 Dependencies
- Add to `Cargo.toml`:
  ```toml
  [dependencies]
  tokio = { version = "1", features = ["full"] }
  warp = "0.3"
  ```

#### 🖥️ Code
```rust
use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
```
- ▶️ Run:
  ```bash
  cargo run
  ```
- 🌐 Access: `http://127.0.0.1:3030/hello/Rust`

---

## **🌟 Phase 4: Real-World Applications**

### 8️⃣ **Build a CLI Tool**
- 🛠️ Use the `clap` crate to build a command-line app.
- Example: A file searcher tool.

### 9️⃣ **Build a REST API**
- 🌐 Use `actix-web` or `warp` to create RESTful APIs.

### 🔟 **Integrate Databases**
- 🗄️ Use `sqlx` or `diesel` for database interactions.

---

## **📘 Resources for Deep Learning**

1️⃣ **Books**
   - 📖 [The Rust Programming Language (Rust Book)](https://doc.rust-lang.org/book/)
   - 📖 [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

2️⃣ **Online Courses**
   - 🖥️ Rust Programming on Udemy, Coursera, etc.

3️⃣ **Practice**
   - 🤝 Contribute to open-source Rust projects.
   - 🛠️ Build small projects like a chat app or a game.
