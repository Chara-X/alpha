use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthCheckRequest {
    object: String,
    check_sub_info: CheckSubInfo,
    health_check_cfg: HealthCheckCfg,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckSubInfo {
    gpu_consistency_check: Vec<String>,
    rdma_consistency_check: Vec<String>,
    gpu_availability_check: Vec<String>,
    rdma_availability_check: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthCheckCfg {
    rdma_nums: i64,
    gpu_nums: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthCheckResponse {
    start_time: String,
    end_time: String,
    gpu_num: i64,
    node_num: i64,
    para_num: i64,
    store_num: i64,
    vendor: String,
    status: String,
    result: Vec<Result>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    check_detail: String,
    check_detail_en: String,
    check_sub_item: String,
    check_type: String,
    end_time: String,
    object: String,
    object_id: Vec<Option<serde_json::Value>>,
    show_detail: bool,
    start_time: String,
    status: String,
    sub_result: String,
}
