use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::error::Error;
use serde::{Deserialize, Serialize};
use serde_json;
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub enabled: bool,
    pub frame_cost: u32,
    pub allow_function_ptrs: bool,
    pub allow_recursion: bool,
    pub tracked_functions: Vec<String>,
}

pub struct Context<'a> {
    pub strack_path: &'a Path,
    pub config: Config,
}

impl Context<'_> {
    pub fn new(strack_path: &Path) -> Result<Context, &str> {
        // Read config file
        let mut bfr = String::new();
        fs::File::open(Context::_config_json_path(strack_path))
            .unwrap()
            .read_to_string(&mut bfr)
            .unwrap();
        let config = serde_json::from_str::<Config>(&bfr).unwrap();

        Ok(Context {
            strack_path: strack_path,
            config: config,
        })
    }

    fn _config_json_path(strack_path: &Path) -> PathBuf {
        strack_path.join("in").join("strack_config.json")
    }
    pub fn config_json_path(&self) -> PathBuf {
        Context::_config_json_path(self.strack_path)
    }
    pub fn su_info_json_path(&self) -> PathBuf {
        self.strack_path.join("local").join("strack_su.json")
    }
    pub fn cg_info_json_path(&self) -> PathBuf {
        self.strack_path.join("local").join("strack_cg.json")
    }
    pub fn node_info_json_path(&self) -> PathBuf {
        self.strack_path.join("out").join("strack_fn_nodes.json")
    }
    pub fn report_json_path(&self) -> PathBuf {
        self.strack_path.join("out").join("strack_report.json")
    }
}
