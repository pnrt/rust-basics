# Rust for Networks and Web Development

## **Introduction**
Rust's safety, concurrency, and performance make it an excellent choice for network programming and web development. This guide provides an overview of libraries, tools, and examples to build robust applications.

---

## **Networking with Rust**

### 1. **Using `std::net` for Basic Networking**

#### Example: TCP Server
```rust
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer));
    stream.write(b"HTTP/1.1 200 OK\r\n\r\nHello, World!").unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server running on 127.0.0.1:7878");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_client(stream);
    }
}
```

### 2. **Asynchronous Networking with Tokio**

#### Example: Async TCP Server
```rust
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Server running on 127.0.0.1:8080");

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let mut buffer = [0; 512];
            socket.read(&mut buffer).await.unwrap();
            println!("Received: {}", String::from_utf8_lossy(&buffer));

            socket.write_all(b"HTTP/1.1 200 OK\r\n\r\nHello, Async World!").await.unwrap();
        });
    }
}
```

---

## **Web Development with Rust**

### 3. **Building a REST API with Actix-Web**

#### Example: Basic REST API
```rust
use actix_web::{get, web, App, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    "Hello, Rust Web!"
}

#[get("/user/{id}")]
async fn get_user(web::Path(id): web::Path<u32>) -> impl Responder {
    format!("User ID: {}", id)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(get_user)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
```

### 4. **Using Warp for Declarative APIs**

#### Example: Hello World API
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

---

## **Integrating with Databases**

### 5. **Using SQLx for Async Database Operations**

#### Example: Connecting to a PostgreSQL Database
```rust
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://user:password@localhost/db_name").await?;

    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(&pool).await?;

    println!("User count: {}", row.0);

    Ok(())
}
```

---

## **WebSocket Communication**

### 6. **Using Tungstenite**

#### Example: Simple WebSocket Server
```rust
use std::net::TcpListener;
use tungstenite::accept;

fn main() {
    let server = TcpListener::bind("127.0.0.1:9001").unwrap();
    println!("WebSocket server running on ws://127.0.0.1:9001");

    for stream in server.incoming() {
        let mut websocket = accept(stream.unwrap()).unwrap();
        websocket.write_message("Hello WebSocket".into()).unwrap();
    }
}
```

---

## **GraphQL APIs**

### 7. **Using Juniper for GraphQL**

#### Example: Simple GraphQL Server
```rust
use juniper::{EmptyMutation, EmptySubscription, FieldResult, RootNode};
use warp::Filter;

struct Query;
#[juniper::graphql_object]
impl Query {
    fn api_version() -> &str {
        "1.0"
    }
}

type Schema = RootNode<'static, Query, EmptyMutation, EmptySubscription>;

#[tokio::main]
async fn main() {
    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);

    let graphql_filter = warp::post()
        .and(warp::path("graphql"))
        .and(juniper_warp::make_graphql_filter(schema, warp::filters::BoxedFilter::new()));

    warp::serve(graphql_filter).run(([127, 0, 0, 1], 8080)).await;
}
```

---

## **API Security and Authentication**

### 8. **JWT Authentication with Actix-Web**

#### Example: Token Generation and Validation
```rust
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

fn generate_token(user_id: &str) -> String {
    let claims = Claims {
        sub: user_id.to_owned(),
        exp: 10000000000, // Example expiration
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref())).unwrap()
}

async fn login() -> impl Responder {
    let token = generate_token("user123");
    HttpResponse::Ok().body(token)
}

async fn validate_token(token: &str) -> bool {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default()
    ).is_ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/login", web::get().to(login))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
```

---

# Advanced API Development Guide

## Authentication with JWT

Secure your API using JSON Web Tokens (JWT) for authentication.

```rust
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

const SECRET: &[u8] = b"secret_key";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/login", web::post().to(login))
            .route("/protected", web::get().to(protected))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn login() -> impl Responder {
    let claims = Claims {
        sub: "user123".to_string(),
        exp: 10000000000, // Expiry timestamp
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET),
    ).unwrap();

    HttpResponse::Ok().body(token)
}

async fn protected(req: actix_web::HttpRequest) -> impl Responder {
    let auth_header = req.headers().get("Authorization");

    if let Some(header_value) = auth_header {
        if let Ok(token) = header_value.to_str() {
            if let Ok(decoded) = decode::<Claims>(
                token.trim_start_matches("Bearer "),
                &DecodingKey::from_secret(SECRET),
                &Validation::default(),
            ) {
                return HttpResponse::Ok().body(format!("Welcome, {}", decoded.claims.sub));
            }
        }
    }

    HttpResponse::Unauthorized().body("Access denied")
}
```

## Encrypting Sensitive Data

Ensure data security using AES encryption.

```rust
use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use hex_literal::hex;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

fn main() {
    let key = hex!("000102030405060708090a0b0c0d0e0f");
    let iv = hex!("101112131415161718191a1b1c1d1e1f");

    let plaintext = b"Secret message";

    // Encrypt
    let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
    let ciphertext = cipher.encrypt_vec(plaintext);
    println!("Ciphertext: {:?}", ciphertext);

    // Decrypt
    let decrypted = cipher.decrypt_vec(&ciphertext).unwrap();
    println!("Decrypted text: {:?}", String::from_utf8(decrypted).unwrap());
}
```

## Middleware for Logging and Monitoring

Add logging capabilities to your application.

```rust
use actix_web::{App, HttpServer, middleware::Logger};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default()) // Logs requests
            .service(actix_web::web::resource("/").to(|| async { "Hello, Middleware!" }))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

## Rate Limiting with Tower

Prevent API abuse by limiting request rates.

```rust
use tower::{ServiceBuilder, limit::RateLimitLayer};
use tokio::time::Duration;
use hyper::{Server, service::make_service_fn, service::service_fn, Body, Request, Response};

#[tokio::main]
async fn main() {
    let rate_limiter = ServiceBuilder::new()
        .layer(RateLimitLayer::new(5, Duration::from_secs(60)))
        .service(service_fn(|_req: Request<Body>| async {
            Ok::<_, hyper::Error>(Response::new(Body::from("Hello, Rate Limiting!")))
        }));

    let make_svc = make_service_fn(|_| {
        let svc = rate_limiter.clone();
        async move { Ok::<_, hyper::Error>(svc) }
    });

    let addr = ([127, 0, 0, 1], 3000).into();
    Server::bind(&addr).serve(make_svc).await.unwrap();
}
```

## Advanced GraphQL with Subscriptions

Implement real-time data updates using GraphQL subscriptions.

```rust
use async_graphql::*;
use async_graphql_warp::{GraphQLSubscription, Subscription};
use warp::Filter;

struct Query;
#[Object]
impl Query {
    async fn hello(&self) -> &str {
        "Hello, GraphQL!"
    }
}

struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn counter(&self) -> impl Stream<Item = i32> {
        tokio_stream::iter(0..).throttle(std::time::Duration::from_secs(1))
    }
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(Query, EmptyMutation, SubscriptionRoot).finish();

    warp::serve(
        warp::path("graphql").and(GraphQLSubscription::new(schema)),
    )
    .run(([127, 0, 0, 1], 8080))
    .await;
}
