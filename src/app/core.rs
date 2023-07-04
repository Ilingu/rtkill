use std::{
    fs,
    sync::mpsc::{self, Sender},
    thread,
};

use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use toml::Table;

use super::{RTKill, TargetDir};

#[derive(Debug)]
enum TraverseMsg {
    Data(TargetDir),
    Exit,
}

impl RTKill {
    pub fn search(&mut self) -> Result<()> {
        let (tx, rx) = mpsc::channel::<TraverseMsg>();

        let path = self.root_dir.clone();
        thread::spawn(move || {
            let _ = find_target_dirs(path, tx.clone());
            let _ = tx.send(TraverseMsg::Exit);
        });
        for data in rx {
            match data {
                TraverseMsg::Data(target) => self.target_directories.datas.push(target),
                TraverseMsg::Exit => break,
            }
        }

        Ok(())
    }
}

fn find_target_dirs(path: String, tx: Sender<TraverseMsg>) -> Result<()> {
    if let Ok(entries) = fs::read_dir(&path) {
        let entries = entries
            .into_iter()
            .filter_map(|r| r.ok())
            .collect::<Vec<_>>();

        let cargo_toml = entries.iter().find(|de| {
            de.file_type().map(|m| m.is_file()).unwrap_or(false) && de.file_name() == "Cargo.toml"
        });
        let target_dir = entries.iter().find(|de| {
            de.file_type().map(|m| m.is_dir()).unwrap_or(false) && de.file_name() == "target"
        });

        // if target dir detected: send it
        if let (Some(cargo_toml), Some(target)) = (cargo_toml, target_dir) {
            let toml_values = fs::read_to_string(cargo_toml.path())?.parse::<Table>()?;
            let package_info = toml_values
                .iter()
                .find(|(key, val)| key == &&"package".to_string() && val.is_table())
                .ok_or(anyhow!("Cannot find package"))?
                .1
                .as_table()
                .ok_or(anyhow!("Cannot parse package"))?;
            let project_name = package_info
                .iter()
                .find(|(key, val)| key == &&"name".to_string() && val.is_str())
                .map(|(_, val)| val.as_str().unwrap().to_string())
                .ok_or(anyhow!(""))?;

            let last_modified: DateTime<Utc> = target.metadata()?.modified()?.into();
            let target_dir = TargetDir {
                path: target.path().to_str().unwrap_or_default().to_string(),
                project_name,
                last_modified: last_modified.format("%d/%m/%Y %T").to_string(),
            };
            let _ = tx.send(TraverseMsg::Data(target_dir));
            return Ok(());
        }

        // otherwise, continue traversal
        for entry in entries {
            if entry.file_type().map(|m| m.is_dir()).unwrap_or(false) {
                let _ = find_target_dirs(
                    entry.path().to_str().unwrap_or_default().to_string(),
                    tx.clone(),
                );
            }
        }
    }
    Ok(())
}
