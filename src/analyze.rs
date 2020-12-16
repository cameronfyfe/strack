use std::fs;
use std::io::Read;
use std::path::Path;
use std::process::Command;

use serde::{Deserialize, Serialize};

use super::config::{Config, Context};
use super::stack_usage;

pub fn analyze(ctx: &Context, args: Vec<&str>) {
    // Create stack usage file
    stack_usage::create_su_info_file_from_o_files(
        &ctx.su_info_json_path(),
        args.iter().map(|&p| Path::new(p)).collect(),
    );
    // let status = Command::new("python3") o_filepaths: &args)
    //     .arg("src/python/su_info.py")
    //     .arg(ctx.su_info_json_path())
    //     .args(&args)
    //     .status()
    //     .expect("process failed to execute");
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
