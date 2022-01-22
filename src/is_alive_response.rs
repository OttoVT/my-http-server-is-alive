use std::collections::HashMap;

use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct IsAliveResponse<'a> {
    pub is_alive: bool,
    pub framework_version: &'a str,
    pub app_version: &'a str,
    pub app_compilation_date: &'a str,
    pub env_info: &'a str,
    pub env_variable_sha1: &'a HashMap<String, String>,
}