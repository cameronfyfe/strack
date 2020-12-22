#[macro_use]
extern crate prettytable;
pub mod analyze;
pub mod call_graph;
pub mod config;
pub mod fn_node;
pub mod report;
pub mod stack_usage;

use std::path::Path;
use std::{self};

use config::Context;
use log::info;

pub fn run(args: Vec<String>) -> i32 {
    let strack_path = Path::new(&args[0]);
    let strack_function = args[1].as_str();
    let strack_args = args[2..].iter().map(AsRef::as_ref).collect::<Vec<&str>>();

    // Read config file, prepare running context
    let ctx = match Context::new(strack_path) {
        Ok(ctx) => ctx,
        Err(_) => panic!("Issue reading strack config."),
    };

    // Check if strack is enabled
    if ctx.config.enabled == false {
        return 1;
    }

    // Init Logger
    flexi_logger::Logger::with_str("info")
        .log_to_file()
        .directory(&ctx.log_file_dir())
        .start_with_specfile(&ctx.default_log_spec_file())
        .expect("Logger failed to initialize.");
    info!("Logger initialized.");

    // Run Strack Function
    match strack_function {
        "analyze" => {
            analyze::analyze(&ctx, strack_args);
        },
        "report" => {
            report::report(&ctx);
        },
        _ => {
            println!("Invalid strack function.");
            return 1;
        },
    }
    // Ok
    0
}
