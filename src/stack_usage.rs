use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::vec::Vec;

use super::fn_node;
use super::fn_node::FnInfo;
use super::fn_node::FnStackUsage;
use super::fn_node::Lang;

pub fn create_su_info_file_from_o_files(su_json_path: &Path, o_filepaths: Vec<&Path>) {
    // Get stack usage info from .su files
    let mut fns = Vec::new();
    for o_filepath in o_filepaths {
        fns.extend(get_stack_usage_from_su_file(o_filepath));
    }

    // Write stack usage info to json format
    let json = serde_json::to_string_pretty(&fns).unwrap();

    // Write stack usage json to file
    fs::create_dir_all(su_json_path.parent().unwrap()).unwrap();
    fs::File::create(su_json_path)
        .unwrap()
        .write(json.as_bytes())
        .unwrap();
}

fn get_stack_usage_from_su_file(o_filepath: &Path) -> Vec<fn_node::FnStackUsage> {
    let mut fns = Vec::new();

    println!("Parsing .su for {}", o_filepath.to_string_lossy());

    // Convert .o filename to .su
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
            return Vec::new();
        }
    };

    // Read .su file
    for line in BufReader::new(su_file).lines() {
        let line =
            line.expect(format!("Unreadable line encountered in {}", su_filepath_str).as_str());
        let lang = if line.contains('(') {
            println!("Cpp Lang");
            Lang::Cpp
        } else {
            println!("C Lang");
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

                        println!("{} {} {}", fn_symbol, stack_usage, stack_usage_type);

                        // UPDATE NAMES HERE SO JSON FILE IS MADE WITH CORRECT FIELDS

                        let fn_su = FnStackUsage {
                            node: FnInfo::new(o_filepath, fn_symbol),
                            local_type: stack_usage_type.to_string(),
                            local_usage: Some(stack_usage),
                            su_local_known: true,
                        };

                        fns.push(fn_su);
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
