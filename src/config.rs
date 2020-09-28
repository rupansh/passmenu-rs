/*
passmenu-rs
Copyright (C) 2020  Rupansh Sekar

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
use crate::consts::*;

use dirs::{config_dir, home_dir};
use serde::{Deserialize};
use std::process::Command;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Deserialize)]
struct DeConfig {
    rofi_args: String,
    pass_cmd: Option<String>,
    pass_dir: Option<String>
}

impl Default for DeConfig {
    fn default() -> DeConfig { DeConfig { rofi_args: "".into(), pass_cmd: None, pass_dir: None } }
}

pub struct AppConfig {
    pub rofi_args: Vec<String>,
    pub pass_cmd: String,
    pub pass_dir: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        return Self {
            rofi_args: Vec::new(),
            pass_cmd: DPASS_CMD.to_string(),
            pass_dir: DPASS_DIR.to_string()
        }
    }
}

pub fn get_conf() -> Result<AppConfig, (String, Vec<String>)> {
    let cond = config_dir();
    if (&cond).is_some() {
        let conf: PathBuf = [cond.unwrap(), PathBuf::from(CONFIG_NAME)].iter().collect();
        if conf.exists() {
            let mut c = File::open(conf.as_path()).unwrap();
            let mut cs = String::new();
            c.read_to_string(&mut cs).unwrap_or_default();
            let dc: DeConfig = toml::from_str(&cs).unwrap();
            let mut app_config = AppConfig::default();
            app_config.rofi_args = dc.rofi_args.split(" ").map(|s| s.to_string()).collect();

            if dc.pass_cmd.is_some() {
                app_config.pass_cmd = dc.pass_cmd.unwrap();
                if Command::new(app_config.pass_cmd.as_str()).arg("--version").output().is_err() {
                    return Err(("Invalid pass command in config!".to_string(), app_config.rofi_args))
                }
            }

            if dc.pass_dir.is_some() {
                app_config.pass_dir = dc.pass_dir.unwrap();

                let pass_dir: PathBuf = match home_dir() {
                    Some(p) => [p, PathBuf::from(app_config.pass_dir.as_str())].iter().collect(),
                    _ => return Err((format!("Failed to find your home directory!"), app_config.rofi_args))
                };

                if !pass_dir.exists() {
                    return Err(("pass directory not found!".to_string(), app_config.rofi_args))
                }
            }

            return Ok(app_config);
        }
    }

    return Ok(AppConfig::default());
}