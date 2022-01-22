use std::collections::HashMap;

pub struct IsAliveContext {
    pub variables: HashMap<String, String>,
    pub is_alive: bool,
    pub framework_version: String,
    pub app_version: String,
    pub app_compilation_date: String,
    pub env_info: String,
}