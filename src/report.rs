use std::fs;
use std::io::Read;
use std::path::Path;

use serde::{Deserialize, Serialize};

use super::config::{Config, Context};

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

impl Report {
    pub fn pct_fns_known_local_stack(&self) -> f32 {
        100.0 * self.num_functions_known_local_stack as f32 / self.total_function_nodes as f32
    }

    pub fn pct_fns_known_max_stack(&self) -> f32 {
        100.0 * self.num_functions_known_max_stack as f32 / self.total_function_nodes as f32
    }
}

pub fn report(ctx: &Context) {
    if ctx.config.enabled == false {
        return;
    }

    // Read report file
    let mut bfr = String::new();
    fs::File::open(ctx.report_json_path())
        .expect(
            format!(
                "Error opening report file for reading: {}",
                ctx.report_json_path().to_string_lossy()
            )
            .as_str(),
        )
        .read_to_string(&mut bfr)
        .expect(
            format!(
                "Error reading data from report file: {}",
                ctx.report_json_path().to_string_lossy()
            )
            .as_str(),
        );
    let report = serde_json::from_str::<Report>(&bfr).expect(
        format!(
            "Error parsing json from report file: {}",
            ctx.report_json_path().to_string_lossy()
        )
        .as_str(),
    );

    // Print report data
    table!(
        ["Strack Report"],
        [table!(
            [
                "Total function nodes",
                report.total_function_nodes.to_string(),
                ""
            ],
            [
                "Functions with known local stack usage",
                report.num_functions_known_local_stack.to_string(),
                format!("{:.1}%", report.pct_fns_known_local_stack())
            ],
            [
                "Functions with known max stack usage",
                report.num_functions_known_max_stack.to_string(),
                format!("{:.1}%", report.pct_fns_known_max_stack())
            ]
        )]
    )
    .printstd();
}
