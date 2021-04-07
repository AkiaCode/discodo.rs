# dscodo.rs
Rust wrapper of Discodo

##### [Docs.rs](https://docs.rs/discodo)

### Example
```rust
#[tokio::main]
async fn main() {

    let framework = StandardFramework::new().configure(|c| c.prefix("/"));

    let mut client = Client::builder("TOKEN")
        .framework(framework)
        .register_discodo("127.0.0.1", None, None).await
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
```