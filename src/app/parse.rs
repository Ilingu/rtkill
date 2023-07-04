use std::{env, fs};

use anyhow::{anyhow, Result};

use super::RTKill;

impl RTKill {
    pub fn from_cd() -> Result<Self> {
        let root_dir = env::current_dir()?
            .to_str()
            .ok_or(anyhow!("Failed to parse path"))?
            .to_string();
        Ok(Self {
            root_dir,
            ..Default::default()
        })
    }

    pub fn from_args() -> Result<Self> {
        let args = env::args().skip(1).collect::<Vec<_>>();
        if args.len() != 1 {
            return Err(anyhow!("bad arg"));
        }

        let root_dir = &args[0];
        if !fs::metadata(root_dir)?.is_dir() {
            return Err(anyhow!("provided path, is not a directory"));
        }

        Ok(Self {
            root_dir: root_dir.to_owned(),
            ..Default::default()
        })
    }
}
