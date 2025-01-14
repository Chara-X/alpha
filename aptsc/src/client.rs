use crate::msg;
use reqwest::blocking;
use std::any::TypeId;
pub struct Client {
    client: blocking::Client,
    addr: String,
}

impl Client {
    pub fn new(client: blocking::Client, addr: String) -> Self {
        Self { client, addr }
    }
    pub fn create<T>(&self, req: &T) {
        // let url = self.host
        //     + match TypeId::of::<T>() {
        //         TypeId::of::<msg::HealthCheckRequest>() => "https://example.com/api/healthcheck",
        //         _ => unreachable!(),
        //     };
    }
}
