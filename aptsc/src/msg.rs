use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HealthCheckRequest {
    pub object: String,
    pub check_sub_info: CheckSubInfo,
    pub health_check_cfg: HealthCheckCfg,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CheckSubInfo {
    pub gpu_consistency_check: Option<Vec<String>>,
    pub rdma_consistency_check: Option<Vec<String>>,
    pub gpu_availability_check: Option<Vec<String>>,
    pub rdma_availability_check: Option<Vec<String>>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HealthCheckCfg {
    pub rdma_nums: i64,
    pub gpu_nums: i64,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HealthCheckResponse {
    pub status: String,
    pub vendor: String,
    pub start_time: String,
    pub end_time: String,
    pub node_num: i64,
    pub gpu_num: i64,
    pub para_num: i64,
    pub store_num: i64,
    pub result: Vec<Result>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub check_detail: String,
    pub check_detail_en: String,
    pub check_sub_item: String,
    pub check_type: String,
    pub end_time: String,
    pub object: String,
    pub show_detail: bool,
    pub start_time: String,
    pub status: String,
    pub sub_result: String,
}
