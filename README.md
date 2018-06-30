# leanpub-rs

An (unofficial) Leanpub API client. 

## Usage

Add a dependency to the `leanpub` crate.

```toml
[dependencies]
leanpub = "0.1.3"
```

Now you should be able to use it after you've imported
the crate in your application or library.

```rust
extern crate leanpub;

fn main() {
    let client = leanpub::Client::new(Option::None);
    let result = client.get_summary("rprogramming").unwrap();

    println!("Title: {}", result.title);
    println!("Minimum price: {}", result.minimum_price);
    println!("Suggested price: {}", result.suggested_price);
}
```
