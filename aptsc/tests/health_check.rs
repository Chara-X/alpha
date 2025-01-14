use aptsc::msg;
use reqwest::blocking::Client;
use std::{env, error};
#[test]
fn test() -> Result<(), Box<dyn error::Error>> {
    // Prepare the request payload
    let req = msg::HealthCheckRequest {
        object: "cluster".to_string(),
        check_sub_info: msg::CheckSubInfo {
            gpu_consistency_check: Some(vec![msg::GpuConsistencyCheck::GpuHardDropNum]),
            rdma_consistency_check: Some(vec![msg::RdmaConsistencyCheck::RdmaPortNum]),
        },
        health_check_cfg: msg::HealthCheckConfig {
            rdma_nums: 2,
            gpu_nums: 4,
        },
    };
    let client = Client::new();
    let res: msg::HealthCheckResponse = client
        .post(env::var("APTS_ADDR").unwrap())
        .json(&req) // Automatically serialize to JSON
        .send()?
        .json()?;
    // Verify the deserialized response
    assert_eq!(
        res.status, "Success",
        "Health check status is not 'Success'"
    );
    println!("Health check response: {:?}", res);
    Ok(())
}
