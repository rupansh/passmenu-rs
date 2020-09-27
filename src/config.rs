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

use dirs::config_dir;
use serde::{Deserialize};

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

#[derive(Default)]
pub struct AppConfig {
    pub rofi_args: String,
    pub pass_cmd: String,
    pub pass_dir: String,
}

pub fn get_conf() -> AppConfig {
    let cond = config_dir();
    if (&cond).is_some() {
        let conf: PathBuf = [cond.unwrap(), PathBuf::from(CONFIG_NAME)].iter().collect();
        if conf.exists() {
            let mut c = File::open(conf.as_path()).unwrap();
            let mut cs = String::new();
            c.read_to_string(&mut cs).unwrap_or_default();
            let dc: DeConfig = toml::from_str(&cs).unwrap();
            let app_config = AppConfig {
                rofi_args: dc.rofi_args,
                pass_cmd: dc.pass_cmd.unwrap_or(DPASS_CMD.to_string()),
                pass_dir: dc.pass_dir.unwrap_or(DPASS_DIR.to_string())
            };

            return app_config;
        }
    }
    return AppConfig::default();
}