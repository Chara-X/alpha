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
/**
```http
POST {{addr}}/opapi/wsm/v1/apts/namespace HTTP/1.1

{
    "nameSpace": "test70b"
}
```
*/
fn main() {
    println!("{}", env::current_exe().unwrap().display());
}
