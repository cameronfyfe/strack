use std::path::Path;
use std::path::PathBuf;
use std::vec::Vec;

use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};
use std::clone::Clone;
#[derive(Clone)]
pub enum Lang {
    C,
    Cpp,
}

impl Serialize for Lang {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(match *self {
            Lang::C => "C",
            Lang::Cpp => "Cpp",
        })
    }
}
impl <'de> Deserialize<'de> for Lang {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "C" => Ok(Lang::C),
            "Cpp" => Ok(Lang::Cpp),
            _ => Err(D::Error::custom(format!("Rejectedt '{}', 'C' or 'Cpp' are only valid lang values", &s)))
        }
    }
}

// TODO: do this better, put this somewhere that makes more sense
fn name_from_symbol(symbol: &str) -> &str {
    // TODO: demangle if Cpp
    symbol
}
#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct FnInfo {
    pub name: String,
    pub symbol: String,
    #[serde(rename = "obj_filepath")]
    pub o_pathbuf: PathBuf,
    #[serde(rename = "obj_filename")]
    pub o_filename: String,
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
            o_filename: o_filepath
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string(),
            lang: Lang::C, // TODO Cpp
            arg_types: Vec::new(),
            return_type: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct FnStackUsage {
    #[serde(rename = "fn_id")]
    pub node: FnInfo,
    #[serde(rename = "su_local_type")]
    pub local_type: String, // What is this ('static')
    #[serde(rename = "su_local")]
    pub local_usage: Option<u32>,
    pub su_local_known: bool,
}

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct FnEdgeInfo {
    #[serde(rename = "fn_id")]
    pub node: FnInfo,
    pub children: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct FnNode {
    #[serde(rename = "fn_id")]
    pub info: FnInfo,
    pub stack_usage: Option<FnStackUsage>,
    pub edge_info: Option<FnEdgeInfo>,
    pub children_missing: Vec<String>,
    pub su_max: u32,
    pub su_max_known: bool, // TODO: remove
    pub su_max_call_path: Vec<String>,

    #[serde(rename = "su_local_type")]
    pub local_type: String, // What is this ('static')
    #[serde(rename = "su_local")]
    pub local_usage: Option<u32>,
    pub su_local_known: bool,

    pub children: Vec<FnEdgeInfo>,
}
