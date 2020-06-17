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

extern crate serde;
extern crate toml;
extern crate dirs;

use crate::consts::*;

use dirs::config_dir;
use serde::{Deserialize};

use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Deserialize)]
struct Config {
    rofi_args: String
}

impl Default for Config {
    fn default() -> Config { Config { rofi_args: "".into() } }
}

pub fn get_conf() -> Vec<String> {
    let cond = config_dir();
    if (&cond).is_some() {
        let conf: PathBuf = [cond.unwrap(), PathBuf::from(CONFIG_NAME)].iter().collect();
        if conf.exists() {
            let mut c = File::open(conf.as_path()).unwrap();
            let mut cs = String::new();
            c.read_to_string(&mut cs).unwrap_or_default();
            let dc: Config = toml::from_str(&cs).unwrap();
            return dc.rofi_args.split(" ").map(|s| s.to_string()).collect();
        }
    }
    return vec!["".to_string()];
}