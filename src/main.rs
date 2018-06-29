extern crate leanpub;

fn main() {
    let client = leanpub::Client::new(Option::None);
    let result = client.get_summary("rprogramming").unwrap();

    println!("Title: {}", result.title);
    println!("Minimum price: {}", result.minimum_price);
    println!("Suggested price: {}", result.suggested_price);
}