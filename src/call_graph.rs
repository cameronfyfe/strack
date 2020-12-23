use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::process::Command;
use std::vec::Vec;
use std::collections::HashMap;

use log::{info, trace, warn};

pub fn get_call_graph_from_o_file(o_filepath: &Path) -> HashMap<String, Vec<String>> {
    let mut fns = HashMap::new();
    // Working node while parsing
    let mut cur_node = None as Option<(String, Vec<String>)>;
    // TODO: do this differently, not at all how I want this but this works for now
    // Not sure if a closure can mutably borrow a capture...
    let mut push_cur_node = |node: &Option<(String, Vec<String>)>| -> Option<(String, Vec<String>)> {
        match node {
            Some(node) => {
                let mut c = node.1.clone();
                cleanup_children(&mut c);
                fns.insert(node.0.to_string(), c);
            },
            None => {
                warn!("Tried to push empty cur_node");
            }
        };
        None
    };

    info!("Parsing .o for {}", o_filepath.to_string_lossy());

    // Filename str for reference later
    let o_filepath_str = o_filepath.to_string_lossy();

    // Generate c disassembly from .o file
    let output = Command::new("arm-none-eabi-objdump")
        .arg("-drw")
        .arg(o_filepath)
        .output()
        // TODO: decided what to do here
        .expect(
            format!(
                "Problem running arm-none-eabi-objdump on {} .",
                o_filepath_str
            )
            .as_str(),
        );
    if !output.status.success() {
        // Don't exit since we might still get useful data from other files
        warn!("Problem running arm-none-eabi-objdump.");
        return fns;
    }

    // Parse .cdasm for call graph info
    for line in BufReader::new(&output.stdout[..]).lines() {
        let line = line.expect(
            format!(
                "Unreadable line encountered in disassembly for {}",
                o_filepath_str
            )
            .as_str(),
        );
        // end of fn
        if !cur_node.is_none() && line.is_empty() {
            cur_node = push_cur_node(&cur_node);
        // start of new fn
        } else if line.contains("00000000 <") {
            let fn_name = name_from_cdasm_symbol_line(&line);
            // skip section symbols
            if fn_name.starts_with('.') {
                continue;
            }
            let fn_name = sanitize_symbol_name(fn_name);
            // TODO: probably handle more things that gcc can do but haven't come up yet
            trace!("*** New Function: {}", &fn_name);
            cur_node = Some((
                fn_name,
                Vec::new(),
            ));
            continue;
        // instruction for jumpiong to another fn
        // TODO: update criteria to catch all branch and link events
        } else if line.contains("f7ff fffe") {
            let callee_name = sanitize_symbol_name(line.split('\t').last().unwrap());
            trace!("*** New Callee: {}", callee_name);
            match cur_node {
                Some(ref mut node) => {
                    node.1.push(callee_name);
                },
                None => {
                    warn!("Callee {} found without working node", callee_name);
                }
            }
        }
    }
    cur_node = push_cur_node(&cur_node); // Add last function in disassembly. MAYBE: handle this better within loop?

    // Return
    fns
}

fn name_from_cdasm_symbol_line(line: &str) -> &str {
    // TODO: log if '<' or '>' aren't found
    &line[line.find('<').unwrap_or(0)+1..line.find('>').unwrap_or(line.len())]
}

fn sanitize_symbol_name(name: &str) -> String {
    // handle constprop clone (functions optimized for a known argument, usually a bool)
    // TODO: actually handle constprop clones properly as separate functions
    if name.contains("constprop") {
        name.split('.')
            .collect::<Vec<&str>>()
            .split_last()
            .unwrap()
            .1
            .iter()
            .map(|s| *s)
            .collect::<Vec<&str>>()
            .join(".")
    } else {
        name.to_string()
    }
}

fn cleanup_children(children: &mut Vec<String>) {
    // remove duplicates for functions call multiple times in same function
    children.sort();
    children.dedup();
}