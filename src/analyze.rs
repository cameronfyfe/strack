use std::fs;
use std::io::Read;
use std::process::Command;

use serde::{Deserialize, Serialize};

use super::config::{Config, Context};

pub fn analyze(ctx: &Context, args: Vec<&str>) {
    // Create stack usage file
    let status = Command::new("python3")
        .arg("src/python/su_info.py")
        .arg(ctx.su_info_json_path())
        .args(&args)
        .status()
        .expect("process failed to execute");
    // Create call graph file
    let status = Command::new("python3")
        .arg("src/python/cg_info.py")
        .arg(ctx.cg_info_json_path())
        .args(&args)
        .status()
        .expect("process failed to execute");
    // Analyze
    let status = Command::new("python3")
        .arg("src/python/cg_su_info.py")
        .arg(ctx.node_info_json_path())
        .arg(ctx.report_json_path())
        .arg(ctx.su_info_json_path())
        .arg(ctx.cg_info_json_path())
        .arg(ctx.config_json_path())
        .args(&args)
        .status()
        .expect("process failed to execute");
}
