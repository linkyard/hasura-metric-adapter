
use reqwest::Response;
use serde::{Deserialize, Serialize};
use log::warn;

#[derive(Serialize)]
pub struct SQLRequest {
    #[serde(rename = "type")]
    pub request_type: String,
    #[serde(rename = "args")]
    pub args: std::vec::Vec<RunSQLQuery>,
}

#[derive(Serialize)]
pub struct RunSQLQuery {
    #[serde(rename = "type")]
    pub request_type: String,
    #[serde(rename = "args")]
    pub args: RunSQLArgs,
}

#[derive(Serialize)]
pub struct RunSQLArgs {
    #[serde(rename = "cascade")]
    pub cascade: bool,
    #[serde(rename = "read_only")]
    pub read_only: bool,
    #[serde(rename = "sql")]
    pub sql: String,
}

#[derive(Deserialize)]
pub struct SQLResult {
    #[serde(rename = "result_type")]
    pub result_type: String,
    #[serde(rename = "result")]
    pub result: std::vec::Vec<std::vec::Vec<String>>,
}

pub(crate) async fn make_sql_request(request: &SQLRequest, cfg: &crate::Configuration) -> reqwest::Result<Response> {
    let client = reqwest::Client::new();
    client
        .post(format!("{}/v2/query", cfg.hasura_addr))
        .json(request)
        .header("x-hasura-admin-secret", cfg.hasura_admin.to_owned())
        .send()
        .await
}
pub(crate) fn get_sql_entry_value(entry: &Vec<String>) -> Option<(i64, Option<String>)> {
    if entry.len() >= 1 && entry.len() <= 2 {
        let str_value = entry.get(0).unwrap();
        if let Some(value) = str_value.parse::<i64>().ok() {
            return Some((value, entry.get(1).map(|v| v.to_owned())));
        } else {
            warn!(
                "Failed to collect scheduled event count result not a number: {}",
                str_value
            );
        }
    }
    None
}


pub(crate) fn get_sql_result_value(result: &SQLResult) -> Option<(i64, Option<String>)> {
    if let Some(entry) = result.result.get(1) {
        return get_sql_entry_value(entry);
    }
    None
}