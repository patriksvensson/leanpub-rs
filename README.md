# leanpub-rs

An (unofficial) Leanpub API client. 

[Crate](https://crates.io/crates/leanpub/0.1.4)  
[Documentation](https://docs.rs/leanpub/0.1.4/leanpub/)

## Usage

Add a dependency to the `leanpub` crate.

```toml
[dependencies]
leanpub = "0.1.4"
```

Now you should be able to use it after you've imported
the crate in your application or library.

```rust
extern crate leanpub;

use std::path::Path;

fn main() {
    let client = leanpub::Client::new(Option::Some("secret-api-key"));
    let result = client.get_summary("my-book").unwrap();

    println!("Title: {}", result.title);
    println!("Minimum price: {}", result.minimum_price);
    println!("Suggested price: {}", result.suggested_price);

    client.download_preview(
        "my-book",
        Path::new("./my_book.pdf"), 
        leanpub::PreviewFormat::Pdf).unwrap();
}
```
