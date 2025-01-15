fn main() {}

pub mod msg;

use reqwest::blocking;
use serde;
#[derive(open_api::OpenApi)]
pub struct Client {
    c: blocking::Client,
    svr: String,
}
impl Client {
    #[open_api(method = "POST", path = "/opapi/wsm/v1/apts/inspection")]
    pub fn create_gpu_inspection_task(
        &self,
        req: CreateGpuInspectionTaskRequest,
    ) -> reqwest::Result<CreateGpuInspectionTaskResponse> {
        todo!()
    }
}
