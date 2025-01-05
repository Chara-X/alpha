#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let a = 1;
    let b = 2;
    let x = add(a, b);
}
// An async function, but it doesn't need to wait for anything.
async fn add(a: u32, b: u32) -> u32 {
    println!("Adding {} and {}", a, b);
    a + b
}
