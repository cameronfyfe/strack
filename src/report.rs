use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FnReportInfo {
    pub name: String,
    pub su_max: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Report {
    pub total_function_nodes: u32,
    pub num_functions_known_local_stack: u32,
    pub num_functions_known_max_stack: u32,
    pub tracked_functions: Vec<FnReportInfo>,
    pub unknown_local_su: Vec<String>,
    pub unknown_max_su: Vec<String>,
    pub missing_children: Vec<String>,
}
