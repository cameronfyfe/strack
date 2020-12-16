use std::path::Path;
use std::path::PathBuf;
use std::vec::Vec;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Lang {
    C,
    Cpp,
}

// TODO: do this better, put this somewhere that makes more sense
fn name_from_symbol(symbol: &str) -> &str {
    // TODO: demangle if Cpp
    symbol
}

#[derive(Serialize, Deserialize)]
pub struct FnInfo {
    pub name: String,
    pub symbol: String,
    pub o_pathbuf: PathBuf,
    pub lang: Lang,
    pub arg_types: Vec<String>,
    pub return_type: String, // TODO: o_name, from PathBuf
}

impl FnInfo {
    pub fn new(o_filepath: &Path, symbol: &str) -> FnInfo {
        FnInfo {
            name: name_from_symbol(symbol).to_string(),
            symbol: symbol.to_string(),
            o_pathbuf: o_filepath.to_path_buf(),
            lang: Lang::C, // TODO Cpp
            arg_types: Vec::new(),
            return_type: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct FnStackUsage {
    pub node: FnInfo,
    pub local_type: String, // What is this ('static')
    pub local_usage: Option<u32>,
}

#[derive(Serialize, Deserialize)]
pub struct FnEdgeInfo {
    pub node: FnInfo,
    pub children: Vec<FnEdgeInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct FnNode {
    pub info: FnInfo,
    pub stack_usage: FnStackUsage,
    pub edge_info: FnEdgeInfo,
    pub children_missing: Vec<String>,
    pub su_max: u32,
    pub su_max_callPathBuf: Vec<String>,
}
