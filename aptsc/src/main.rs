use reqwest::blocking::{Client, RequestBuilder};
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{self, ErrorKind};

const BASE_URL: &str = "https://10.166.209.110";

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthCheckRequest {
    pub object: String,
    pub check_sub_info: Option<CheckSubInfo>,
    pub health_check_cfg: Option<HealthCheckCfg>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckSubInfo {
    pub gpu_consistency_check: Option<Vec<String>>,
    pub rdma_consistency_check: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthCheckCfg {
    pub rdma_nums: Option<i32>,
    pub gpu_nums: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthCheckResponse {
    pub result: Option<Vec<HealthCheckResult>>,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthCheckResult {
    pub object: String,
    pub check_type: String,
    pub check_sub_item: String,
    pub sub_result: String,
    pub check_detail: String,
    pub status: String,
    pub start_time: String,
    pub end_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GpuInspectionTaskRequest {
    pub diag_configs: Vec<DiagConfig>,
    pub inspect_count: i32,
    pub inspect_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagConfig {
    pub vendor: String,
    pub diag_param: String,
    pub diag_level: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GpuInspectionTaskResponse {
    pub diag_created: String,
    pub diag_gpu_outputs: Vec<GpuOutput>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GpuOutput {
    pub diag_exec_errmsg: String,
    pub gpu_uuid: String,
    pub result: String,
    pub vendor: String,
}

pub struct ApiClient {
    client: Client,
}

impl ApiClient {
    pub fn new() -> Self {
        let client = Client::builder().build().expect("Failed to create client");
        ApiClient { client }
    }

    fn create_request(&self, method: &str, path: &str) -> RequestBuilder {
        let url = format!("{}{}", BASE_URL, path);
        match method {
            "GET" => self.client.get(&url),
            "POST" => self.client.post(&url),
            "DELETE" => self.client.delete(&url),
            _ => panic!("Unsupported HTTP method"),
        }
    }

    pub fn create_health_check(&self, request: HealthCheckRequest) -> Result<(), io::Error> {
        let response = self
            .create_request("POST", "/opapi/wsm/v1/apts/healthcheck")
            .header(CONTENT_TYPE, "application/json")
            .json(&request)
            .send()
            .map_err(|e| io::Error::new(ErrorKind::Other, e.to_string()))?;

        if response.status().is_success() {
            println!("Health check created successfully");
            Ok(())
        } else {
            let error_msg: HashMap<String, String> = response.json().unwrap_or_default();
            Err(io::Error::new(
                ErrorKind::Other,
                format!(
                    "Error: {}",
                    error_msg
                        .get("message")
                        .unwrap_or(&"Unknown error".to_string())
                ),
            ))
        }
    }

    pub fn get_health_check_result(&self) -> Result<HealthCheckResponse, io::Error> {
        let response = self
            .create_request("GET", "/opapi/wsm/v1/apts/healthcheck")
            .send()
            .map_err(|e| io::Error::new(ErrorKind::Other, e.to_string()))?;

        if response.status().is_success() {
            Ok(response.json().unwrap_or_default())
        } else {
            let error_msg: HashMap<String, String> = response.json().unwrap_or_default();
            Err(io::Error::new(
                ErrorKind::Other,
                format!(
                    "Error: {}",
                    error_msg
                        .get("message")
                        .unwrap_or(&"Unknown error".to_string())
                ),
            ))
        }
    }

    pub fn create_gpu_inspection_task(
        &self,
        request: GpuInspectionTaskRequest,
    ) -> Result<GpuInspectionTaskResponse, io::Error> {
        let response = self
            .create_request("POST", "/opapi/wsm/v1/apts/gpuinspectiontask")
            .header(CONTENT_TYPE, "application/json")
            .json(&request)
            .send()
            .map_err(|e| io::Error::new(ErrorKind::Other, e.to_string()))?;

        if response.status().is_success() {
            Ok(response.json().unwrap_or_default())
        } else {
            let error_msg: HashMap<String, String> = response.json().unwrap_or_default();
            Err(io::Error::new(
                ErrorKind::Other,
                format!(
                    "Error: {}",
                    error_msg
                        .get("message")
                        .unwrap_or(&"Unknown error".to_string())
                ),
            ))
        }
    }

    pub fn delete_gpu_inspection_task(&self) -> Result<(), io::Error> {
        let response = self
            .create_request("DELETE", "/opapi/wsm/v1/apts/gpuinspectiontask")
            .send()
            .map_err(|e| io::Error::new(ErrorKind::Other, e.to_string()))?;

        if response.status().is_success() {
            println!("GPU inspection task deleted successfully");
            Ok(())
        } else {
            let error_msg: HashMap<String, String> = response.json().unwrap_or_default();
            Err(io::Error::new(
                ErrorKind::Other,
                format!(
                    "Error: {}",
                    error_msg
                        .get("message")
                        .unwrap_or(&"Unknown error".to_string())
                ),
            ))
        }
    }

    pub fn get_gpu_inspection_result(&self) -> Result<GpuInspectionTaskResponse, io::Error> {
        let response = self
            .create_request("GET", "/opapi/wsm/v1/apts/gpuinspectiontask")
            .send()
            .map_err(|e| io::Error::new(ErrorKind::Other, e.to_string()))?;

        if response.status().is_success() {
            Ok(response.json().unwrap_or_default())
        } else {
            let error_msg: HashMap<String, String> = response.json().unwrap_or_default();
            Err(io::Error::new(
                ErrorKind::Other,
                format!(
                    "Error: {}",
                    error_msg
                        .get("message")
                        .unwrap_or(&"Unknown error".to_string())
                ),
            ))
        }
    }
}

fn main() {
    let api_client = ApiClient::new();

    // Example usage for creating a health check
    let request = HealthCheckRequest {
        object: "cluster".to_string(),
        check_sub_info: Some(CheckSubInfo {
            gpu_consistency_check: Some(vec!["gpuharddropnum".to_string()]),
            rdma_consistency_check: None,
        }),
        health_check_cfg: Some(HealthCheckCfg {
            rdma_nums: Some(10),
            gpu_nums: Some(10),
        }),
    };

    if let Err(e) = api_client.create_health_check(request) {
        eprintln!("Failed to create health check: {}", e);
    }

    // Example usage for getting health check results
    match api_client.get_health_check_result() {
        Ok(result) => println!("Health check result: {:?}", result),
        Err(e) => eprintln!("Failed to get health check result: {}", e),
    }

    // Example usage for creating a GPU inspection task
    let gpu_request = GpuInspectionTaskRequest {
        diag_configs: vec![DiagConfig {
            vendor: "biren".to_string(),
            diag_param: "".to_string(),
            diag_level: "short".to_string(),
        }],
        inspect_count: 1,
        inspect_type: "GPU".to_string(),
    };

    match api_client.create_gpu_inspection_task(gpu_request) {
        Ok(response) => println!("GPU inspection task created: {:?}", response),
        Err(e) => eprintln!("Failed to create GPU inspection task: {}", e),
    }

    // Example usage for deleting a GPU inspection task
    if let Err(e) = api_client.delete_gpu_inspection_task() {
        eprintln!("Failed to delete GPU inspection task: {}", e);
    }

    // Example usage for getting GPU inspection results
    match api_client.get_gpu_inspection_result() {
        Ok(response) => println!("GPU inspection result: {:?}", response),
        Err(e) => eprintln!("Failed to get GPU inspection result: {}", e),
    }
}
