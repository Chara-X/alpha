use aptsc::msg;
use reqwest::blocking::Client;
use std::error;
#[test]
fn test() -> Result<(), Box<dyn error::Error>> {
    // Prepare the request payload
    let request_payload = msg::HealthCheckRequest {
        object: "test_object".to_string(),
        check_sub_info: msg::CheckSubInfo {
            gpu_consistency_check: Some(vec!["check1".to_string()]),
            rdma_consistency_check: Some(vec!["check2".to_string()]),
            gpu_availability_check: Some(vec!["check3".to_string()]),
            rdma_availability_check: Some(vec!["check4".to_string()]),
        },
        health_check_cfg: msg::HealthCheckCfg {
            rdma_nums: 2,
            gpu_nums: 4,
        },
    };
    // Make the HTTP request
    let url = "https://example.com/api/healthcheck"; // Replace with the actual URL
    let client = Client::new();
    let response = client
        .post(url)
        .json(&request_payload) // Automatically serialize to JSON
        .send();
    // Check if the request was successful
    assert!(response.is_ok(), "Request failed");
    let response = response?.error_for_status()?;
    // Automatically deserialize the response payload
    let health_check_response: msg::HealthCheckResponse = response.json()?;
    // Verify the deserialized response
    assert_eq!(
        health_check_response.status, "Success",
        "Health check status is not 'Success'"
    );
    println!("Health check response: {:?}", health_check_response);
    Ok(())
}
