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
    pub o_pathbuf: PathBuf,
    pub o_filename: String,
    pub lang: Lang,
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
        }
    }
}

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct LocalStackInfo {
    pub stype: String, // What is this ('static')
    pub usage: u32,
}

#[derive(Clone)]
#[derive(Serialize)]
pub struct MaxStackInfo {
    pub usage: u32,
    pub call_path: Vec<String>
}

#[derive(Clone)]
#[derive(Serialize)]
pub struct FnNode {
    pub info: FnInfo,
    pub children: Vec<String>,
    pub local_stack: Option<LocalStackInfo>,
    pub max_stack: Option<MaxStackInfo>,
    pub children_missing: Vec<String>,
}
