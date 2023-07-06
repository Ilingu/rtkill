use std::{
    fs,
    sync::mpsc::{self, Sender},
    thread,
    time::Duration,
};

use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use toml::Table;

use crate::{
    ui::components::message::{Message, MessageType},
    utils::{bytes_len_to_string_prefix, sharable_state::SharableState},
};

use super::{AppState, TargetDir};

#[derive(Debug)]
enum TraverseMsg {
    Data((TargetDir, u64)),
    Exit,
}

impl SharableState<AppState> {
    pub fn push_to_list(&self, target: TargetDir) {
        self.mutate(|data| data.target_directories.datas.push(target));
    }

    pub fn clear_list(&self) {
        self.mutate(|data| data.target_directories.datas.clear());
    }

    pub fn set_searching(&self, searching: bool) {
        self.mutate(|data| data.searching = searching);
    }

    pub fn prev_item(&self) {
        self.mutate(|data| data.target_directories.previous())
    }

    pub fn next_item(&self) {
        self.mutate(|data| data.target_directories.next())
    }

    pub fn delete_current_item(&self) {
        let (sender, receiver) = mpsc::channel::<bool>();
        self.mutate(|data| {
            let current_idx = data.target_directories.index;
            let is_deleted = data.target_directories.datas[current_idx].delete().is_ok();
            let _ = sender.send(is_deleted);
        });

        match receiver.recv_timeout(Duration::from_millis(500)) {
            Ok(is_ok) => match is_ok {
                true => self.set_message(Some(Message::new(
                    "Successfully deleted folder",
                    MessageType::Success,
                    Some(Duration::from_secs(3)),
                    None,
                ))),
                false => self.set_message(Some(Message::new(
                    "Failed to delete folder, try again...",
                    MessageType::Error,
                    None,
                    None,
                ))),
            },
            Err(_) => self.set_message(Some(Message::new(
                "Failed to delete folder, try again...",
                MessageType::Error,
                None,
                None,
            ))),
        };
    }

    pub fn set_message(&self, message: Option<Message>) {
        self.mutate(|data| data.message = message)
    }

    pub fn set_total_size(&self, val: String) {
        self.mutate(|data| data.total_size = val)
    }

    pub fn search(&self) {
        self.set_searching(true);
        let (tx, rx) = mpsc::channel::<TraverseMsg>();

        let path = match &self.read().root_dir {
            Some(path) => path.to_owned(),
            None => {
                self.set_searching(false);
                return;
            }
        };
        thread::spawn(move || {
            find_target_dirs(path, tx.clone());
            let _ = tx.send(TraverseMsg::Exit);
        });

        let (mut is_empty, mut total_size) = (true, 0);
        for data in rx {
            match data {
                TraverseMsg::Data((target, size)) => {
                    self.push_to_list(target);
                    is_empty = false;
                    total_size += size;
                }
                TraverseMsg::Exit => break,
            }
        }

        if is_empty {
            self.set_message(Some(Message::new(
                "There is no 'target' directories in this scope",
                MessageType::Warning,
                None,
                None,
            )));
        }

        self.set_total_size(bytes_len_to_string_prefix(total_size));
        self.set_searching(false);
    }
}

fn find_target_dirs(path: String, tx: Sender<TraverseMsg>) {
    // thread::sleep(Duration::from_millis(1));
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
            thread::scope(|s| {
                s.spawn(move || -> Result<()> {
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

                    let path = target.path().to_str().unwrap_or_default().to_string();
                    let metadata = target.metadata()?;

                    let last_modified: DateTime<Utc> = metadata.modified()?.into();

                    let folder_size = fs_extra::dir::get_size(&path)?;
                    let formated_size = bytes_len_to_string_prefix(folder_size);

                    let target_dir = TargetDir {
                        path,
                        project_name,
                        last_modified: last_modified.format("%d/%m/%Y").to_string(),
                        is_deleted: false,
                        size: formated_size,
                    };
                    let _ = tx.send(TraverseMsg::Data((target_dir, folder_size)));
                    Ok(())
                });
            });
            return;
        }

        // otherwise, continue traversal
        for entry in entries {
            if entry.file_type().map(|m| m.is_dir()).unwrap_or(false) {
                find_target_dirs(
                    entry.path().to_str().unwrap_or_default().to_string(),
                    tx.clone(),
                );
            }
        }
    }
}
