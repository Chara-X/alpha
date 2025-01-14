use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HealthCheckRequest {
    pub object: String,
    pub check_sub_info: CheckSubInfo,
    pub health_check_cfg: HealthCheckConfig,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CheckSubInfo {
    pub gpu_consistency_check: Option<Vec<GpuConsistencyCheck>>,
    pub rdma_consistency_check: Option<Vec<RdmaConsistencyCheck>>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum GpuConsistencyCheck {
    GpuHardDropNum,
    GpuSoftDropNum,
    GpuTopo,
    GpuDriver,
    GpuLinkSpeed,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum RdmaConsistencyCheck {
    RdmaPortNum,
    RdmaMtu,
    RdmaParaVer,
    RdmaStoreVer,
    RdmaResource,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HealthCheckConfig {
    pub rdma_nums: u32,
    pub gpu_nums: u32,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HealthCheckResponse {
    pub result: Vec<CheckResult>,
    pub status: String,
    pub start_time: DateTime<FixedOffset>,
    pub end_time: DateTime<FixedOffset>,
    pub vendor: String,
    pub node_num: u32,
    pub gpu_num: u32,
    pub para_num: Option<u32>,
    pub store_num: Option<u32>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CheckResult {
    pub object: String,
    pub object_id: Vec<String>,
    pub check_type: String,
    pub check_sub_item: String,
    pub sub_result: String,
    pub check_detail: String,
    pub check_detail_en: String,
    pub status: String,
    pub start_time: DateTime<FixedOffset>,
    pub end_time: DateTime<FixedOffset>,
    pub show_detail: bool,
}
