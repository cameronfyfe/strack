use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;
use serde::{de, Deserialize, Serialize};
use std::path::PathBuf;
use super::call_graph;
use super::config::{Config, Context};
use super::stack_usage;
use std::collections::HashMap;
use super::fn_node::*;

pub fn analyze(ctx: &Context, args: Vec<&str>) {
    let o_filepaths = args.iter().map(|&p| Path::new(p)).collect::<Vec<&Path>>();

    let mut fns = Vec::new();

    // Get stack usage info from .su files
    for o_filepath in o_filepaths {
        let mut fns_stack = stack_usage::get_stack_usage_from_su_file(o_filepath);
        let mut fns_callgraph = call_graph::get_call_graph_from_o_file(o_filepath);

        let mut processed_fns = Vec::new();
        // Process functions that have stack usage info
        for (symbol, stack) in &fns_stack {
            processed_fns.push(symbol.to_string());
            fns.push(FnNode {
                info: FnInfo::new(o_filepath, symbol.as_str()),
                children: match fns_callgraph.get(symbol.as_str()) {
                    Some(c) => c.to_vec(),
                    None => Vec::new(),
                },
                local_stack: Some(fns_stack.get(symbol.as_str()).unwrap().clone()),
                max_stack: None,
                children_missing: Vec::new(),
            });
        }
        // Remove processed functions so far
        for symbol in processed_fns {
            fns_stack.remove(symbol.as_str());
            fns_callgraph.remove(symbol.as_str());
        }
        // Process functions in callgraph but missing stack usage info
        for (symbol, children) in &fns_callgraph {
            println!("*** {}", symbol);
            fns.push(FnNode {
                info: FnInfo::new(o_filepath, symbol.as_str()),
                children: match fns_callgraph.get(symbol.as_str()) {
                    Some(c) => c.to_vec(),
                    None => Vec::new(),
                },
                local_stack: None,
                max_stack: None,
                children_missing: Vec::new(),
            })
        }
    }

    // Write json
    let json = serde_json::to_string_pretty(&fns).unwrap();

    // Write
    println!("Writing su json to {}.", &ctx.su_info_json_path().to_string_lossy());
    fs::create_dir_all(&ctx.su_info_json_path().parent().unwrap()).unwrap();
    fs::File::create(&ctx.su_info_json_path())
        .unwrap()
        .write(json.as_bytes())
        .unwrap();

    // Analyze
    analyze_nodes(ctx);
}

// fn get_full_node_from_parts(cg: &FnStackUsage, ) -> Vec<FnNode> {

//     // This node is a sink node on call graph (no further function calls)
//     // so max stack usage is local stack usage
    
// }

fn analyze_nodes(ctx: &Context) {
    // let cg_fns = vec_from_json_file::<FnEdgeInfo>(&ctx.cg_info_json_path());
    // let su_fns = vec_from_json_file::<FnStackUsage>(&ctx.su_info_json_path());

    let fns: Vec<FnNode> = Vec::new();

    // for cg_fn in &cg_fns {

    //     fns.extend(get_full_node_from_parts(cg_fn, fns).iter());





    //     let fnn = FnNode {
    //         info: cg_fn.node.clone(),
    //         edge_info: cg_fn.clone(),
    //         stack_usage: su_fns.iter()
    //         .find(|&f| f.node.symbol == cg_fn.node.symbol)
    //         .cloned(),
    //         children_missing: Vec::new(),
    //         su_max: 0,
    //         su_max_known: false,
    //         su_max_call_path: Vec::new(),
    //     };
    // }
}

fn vec_from_json_file<T: de::DeserializeOwned>(file: &PathBuf) -> Vec<T> {
    obj_from_json_file::<Vec<T>>(file)
}

fn obj_from_json_file<T: de::DeserializeOwned>(file: &PathBuf) -> T {
    let mut bfr = String::new();
    fs::File::open(file)
        .expect(
            format!(
                "Error opening file for reading: {}",
                file.to_string_lossy()
            )
            .as_str(),
        )
        .read_to_string(&mut bfr)
        .expect(
            format!(
                "Error reading data from file: {}",
                file.to_string_lossy()
            )
            .as_str(),
        );
    serde_json::from_str::<T>(&bfr).expect(
        format!(
            "Error parsing json from file: {}",
            file.to_string_lossy()
        )
        .as_str(),
    )
}