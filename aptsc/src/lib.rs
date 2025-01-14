pub mod msg;

use reqwest::blocking;
use serde;
pub struct Client {
    c: blocking::Client,
    svr: String,
}

impl Client {
    pub fn new(c: blocking::Client, svr: String) -> Self {
        Self { c, svr }
    }
    /**
    Update namespace config.

    ## Examples

    ```http
    POST {{svr}}/opapi/wsm/v1/apts/namespace HTTP/1.1

    {
        "nameSpace": "test70b"
    }
    ```
     */
    pub fn update_namespace_config(&self, ns: String) -> reqwest::Result<()> {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Request {
            name_space: String,
        }
        let _response = self
            .c
            .post(format!("{}/opapi/wsm/v1/apts/namespace", self.svr))
            .json(&Request { name_space: ns })
            .send()?;
        Ok(())
    }
    /**
    Create gpu inspection task.

    ## Examples

    Request:

    ```http
    POST {{svr}}/opapi/wsm/v1/apts/gpuinspectiontask HTTP/1.1

    {
    "diag_configs": [
        {
        "vendor": "biren",
        "diag_level": "short"
        }
    ],
    "inspect_count": 1,
    "inspect_type": "GPU",
    }
    ```
     */
    pub fn create_gpu_inspection_task(&self, vender: String, level: String) -> reqwest::Result<()> {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Request {
            inspect_count: i32,
            inspect_type: String,
            diag_configs: Vec<DiagConfig>,
        }
        #[derive(serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct DiagConfig {
            vendor: String,
            diag_level: String,
        }
        let _response = self
            .c
            .post(format!("{}/opapi/wsm/v1/apts/gpuinspectiontask", self.svr))
            .json(&Request {
                inspect_count: 1,
                inspect_type: "GPU".to_string(),
                diag_configs: vec![DiagConfig {
                    vendor: vender,
                    diag_level: level,
                }],
            })
            .send()?;
        Ok(())
    }
    /**
    Delete gpu inspection task.

    ## Examples

    Request:

    ```http
    DELETE {{srv}}/opapi/wsm/v1/apts/gpuinspectiontask HTTP/1.1
    ```
     */
    pub fn delete_gpu_inspection_task(&self) -> reqwest::Result<()> {
        let _response = self
            .c
            .delete(format!("{}/opapi/wsm/v1/apts/gpuinspectiontask", self.svr))
            .send()?;
        Ok(())
    }
    /**
    Get gpu inspection task.

    ## Examples

    Request:

    ```http
    GET {{srv}}/opapi/wsm/v1/apts/gpuinspectiontask HTTP/1.1
    ```

    Response:

    ```http
    HTTP/1.1 200 OK

    {
    "diag_created": "2025-01-14T16:20:10+08:00",
    "diag_gpu_outputs": [
        {
        "diag_exec_errmsg": "GPU is occupied",
        "diag_fail_count": 1,
        "gpuUUID": "NB4B44LQ069",
        "gpu_result_summary": {},
        "model": "Biren106M",
        "node_name": "cluster-cim2-minion-0-3",
        "result": "Undetected",
        "vendor": "BIREN"
        },
    ],
    "diag_total_count": 1,
    "inspect_type": "GPU",
    "status": "running",
    "task_result_summary": {
        "totalCards": 40,
        "totalChecks": 23,
        "totalUsed": 17
    }
    }
    ```
     */
    pub fn get_gpu_inspection_task(&self) -> reqwest::Result<Response> {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Response {
            inspect_type: String,
            status: String,
            task_result_summary: TaskResultSummary,
            diag_total_count: i32,
            diag_created: String,
            diag_gpu_outputs: Vec<DiagGpuOutput>,
        }
        #[derive(serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct TaskResultSummary {
            total_cards: i32,
            total_checks: i32,
            total_used: i32,
        }
        #[derive(serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct DiagGpuOutput {
            node_name: String,
            gpu_uuid: String,
            vendor: String,
            model: String,
            result: String,
            diag_exec_errmsg: String,
            diag_fail_count: i32,
        }
        let _response = self.c.get(&self.svr).send()?;
    }
}
