use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use serde::{de, Deserialize, Serialize};

use super::call_graph;
use super::config::{Config, Context};
use super::fn_node::*;
use super::stack_usage;

enum ComputedMaxStack {
    Value(Option<MaxStackInfo>),
    AlreadyComputed,
}

pub fn analyze(ctx: &Context, args: Vec<&str>) {
    let o_filepaths = args.iter().map(|&p| Path::new(p)).collect::<Vec<&Path>>();

    let mut fns = get_stack_usage_and_call_graph_info(&o_filepaths);

    // Write json
    let json = serde_json::to_string_pretty(&fns).unwrap();

    // Write
    println!(
        "Writing su json to {}.",
        &ctx.su_info_json_path().to_string_lossy()
    );
    fs::create_dir_all(&ctx.su_info_json_path().parent().unwrap()).unwrap();
    fs::File::create(&ctx.su_info_json_path())
        .unwrap()
        .write(json.as_bytes())
        .unwrap();

    // Analyze
    analyze_nodes(ctx, &mut fns);
}

fn get_stack_usage_and_call_graph_info(o_filepaths: &Vec<&Path>) -> Vec<FnNode> {
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
                children_ids: None,
                children_missing: None,
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
                children_ids: None,
                children_missing: None,
            })
        }
    }
    // Return
    fns
}

fn analyze_nodes(ctx: &Context, fns: &mut Vec<FnNode>) {
    populate_children_ids(fns);
    for i in 0..fns.len() {
        match compute_max_stack(&fns[i].clone(), fns, Vec::new()) {
            ComputedMaxStack::Value(max_stack) => {
                fns[i].max_stack = Some(max_stack);
            }
            ComputedMaxStack::AlreadyComputed => {}
        }
        
    }
    
}

fn compute_max_stack<'a>(f: &'a FnNode, fns: &mut Vec<FnNode>, mut callpath: Vec<&'a str>) -> ComputedMaxStack {
    // Already computed
    if f.max_stack.is_some() {
        return ComputedMaxStack::AlreadyComputed;
    }
    // This node is a sink node on call graph (no further function calls)
    // so max stack usage is local stack usage
    if f.children.len() == 0 {
        return match &f.local_stack {
            Some(local_stack) => ComputedMaxStack::Value(Some(MaxStackInfo {
                known: true,
                usage: local_stack.usage,
                call_path: Vec::new(),
            })),
            None => ComputedMaxStack::Value(None),
        };
    }
    // Use max of child nodes
    // let mut max_child_fn = None;
    for c in &f.children_ids {
        let child_callpath = callpath.push(&f.info.name);
    }
    
    ComputedMaxStack::Value(None)
}

fn populate_children_ids(fns: &mut Vec<FnNode>) {
    for i in 0..fns.len() {
        fns[i].children_ids = Some(Vec::new());
        for c in &fns[i].children.clone() {
            match fns.iter().position(|i| c == &i.info.name) {
                Some(id) => {
                    &fns[i].children_ids.as_mut().unwrap().push(id);
                }
                None => {
                    fns[i].children_missing.as_mut().unwrap().push(c.to_string());
                }
            };
        }
    }
}

fn vec_from_json_file<T: de::DeserializeOwned>(file: &PathBuf) -> Vec<T> {
    obj_from_json_file::<Vec<T>>(file)
}

fn obj_from_json_file<T: de::DeserializeOwned>(file: &PathBuf) -> T {
    let mut bfr = String::new();
    fs::File::open(file)
        .expect(format!("Error opening file for reading: {}", file.to_string_lossy()).as_str())
        .read_to_string(&mut bfr)
        .expect(format!("Error reading data from file: {}", file.to_string_lossy()).as_str());
    serde_json::from_str::<T>(&bfr)
        .expect(format!("Error parsing json from file: {}", file.to_string_lossy()).as_str())
}
