use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
enum Status {
    #[serde(rename = "ok")]
    Success,
    #[serde(rename = "err")]
    Failure,
}

fn main() {}
