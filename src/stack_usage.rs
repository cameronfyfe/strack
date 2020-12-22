use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::vec::Vec;
use log::{info, trace, warn};
use super::fn_node::{LocalStackInfo, Lang};

use std::collections::HashMap;

pub fn get_stack_usage_from_su_file(o_filepath: &Path) -> HashMap<String, LocalStackInfo> {
    let mut fns = HashMap::new();

    info!("Parsing .su for {}", o_filepath.to_string_lossy());

    // .su filename from .o filename
    let mut su_filepath = o_filepath.to_path_buf();
    su_filepath.set_extension("su");

    // Filename strs for reference later
    let o_filepath_str = o_filepath.to_string_lossy();
    let su_filepath_str = su_filepath.to_string_lossy();

    // Open .su file
    let su_file = match fs::File::open(&su_filepath) {
        Ok(su_file) => su_file,
        Err(_) => {
            // TODO: rules for which files can be missing .su files.
            // probably only asm files
            println!("Missing {} for {}.", su_filepath_str, o_filepath_str);
            return HashMap::new();
        }
    };

    // Read .su file
    for line in BufReader::new(su_file).lines() {
        let line =
            line.expect(format!("Unreadable line encountered in {}", su_filepath_str).as_str());
        let lang = if line.contains('(') {
            Lang::Cpp
        } else {
            Lang::C
        };
        match lang {
            Lang::Cpp => {
                // TODO
                panic!("Cpp is currently not implemented.");
            }
            Lang::C => {
                let cols = line.split('\t').collect::<Vec<&str>>();
                match cols.len() {
                    3 => {
                        let fn_symbol = cols[0].split(':').last().unwrap();
                        let stack_usage = cols[1].parse::<u32>().unwrap();
                        let stack_usage_type = cols[2]; // TODO: seems like this is always "static", what are other types?

                        trace!("{} {} {}", fn_symbol, stack_usage, stack_usage_type);

                        let fn_su = LocalStackInfo {
                            stype: stack_usage_type.to_string(),
                            usage: stack_usage,
                        };
                        fns.insert(fn_symbol.to_string(), fn_su);
                    }
                    _ => {
                        // Skip
                        // MAYBE: add handling for weird lines, but for now just assume anything without 3 columns is an empty line and can be skipped
                    }
                }
            }
        }
    }

    // Return
    fns
}
