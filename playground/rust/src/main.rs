use std::env;

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
enum Status {
    #[serde(rename = "ok")]
    Success,
    #[serde(rename = "err")]
    Failure,
}
#[derive(Clone)]
struct User {
    name: String,
    age: u32,
    addr: Address,
}
#[derive(Clone)]
struct Address {
    city: String,
}
fn main() {
    println!("{}", env::current_exe().unwrap().display());
}
