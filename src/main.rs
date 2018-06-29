extern crate leanpub;

fn main() {
    let client = leanpub::Client::new("rprogramming", Option::None);
    let result = client.get_summary().unwrap();

    println!("Title: {}", result.title);
    println!("Minimum price: {}", result.minimum_price);
    println!("Suggested price: {}", result.suggested_price);
}