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

mod config;
mod consts;

use config::AppConfig;
use dirs::home_dir;
use once_cell::sync::OnceCell;
use rustofi::{
    components::*,
    RustofiResult,
};
use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};


static APASS_CMD: OnceCell<String> = OnceCell::new();
type GPassCmd = String;
trait GetGlobal {
    fn global() -> &'static String;
}

impl GetGlobal for GPassCmd {
    fn global() -> &'static String {
        return APASS_CMD.get().expect("INVALID PASS_CMD USAGE!! REPORT TO DEV!!")
    }
}

fn main() {
    loop {
        match app_main() {
            _ => break
        }
    }
}

fn app_main() -> RustofiResult {
    let mut app_config = config::get_conf();
    let args = env::args().collect::<Vec<String>>();

    APASS_CMD.set(app_config.pass_cmd.clone()).unwrap();

    for arg in args.iter() {
        if arg == "new" || arg == "ins" {
            match app_config.rofi_args.iter().position(|r| r == "-lines") {
                Some(i) => app_config.rofi_args[i+1] = "0".to_string(),
                _ => { 
                    app_config.rofi_args.push("-lines".to_string());
                    app_config.rofi_args.push("0".to_string())
                }
            }

            return if arg == "new" { pass_generate(app_config) } else { pass_insert(app_config) }
        } else if arg == "del" {
            return pass_delete(app_config)
        }
    }

    return pass_get(app_config)
}

fn pass_generate(app_config: AppConfig) -> RustofiResult {
    return passempty_window(
        &app_config,
        "pass generate",
        |app_config: &AppConfig, sel_val: &str, ()| -> RustofiResult {
            Command::new(app_config.pass_cmd.as_str()).arg("generate").arg("--clip").arg(sel_val).stdout(Stdio::null()).spawn().expect("FAILED TO GENERATE");
            println!("Password Generated and copied to clipboard!");
            return RustofiResult::Success;
        },
        ()
    );
}

fn pass_insert(app_config: AppConfig) -> RustofiResult {
    return passempty_window(
        &app_config,
        "pass insert",
        |app_config: &AppConfig, sel_val: &str, ()| -> RustofiResult {
            return passempty_window(
                app_config,
                "Enter password to insert",
                |app_config: &AppConfig, sel_val: &str, prev_val: &str| -> RustofiResult {
                    let p_ins = Command::new(app_config.pass_cmd.as_str()).arg("insert").arg(prev_val).stdin(Stdio::piped()).stdout(Stdio::null()).spawn().expect("FAILED TO INSERT");
                    writeln!(p_ins.stdin.unwrap(), "{}\n{}", sel_val, sel_val).unwrap();
                    println!("Password added!");
                    return RustofiResult::Success;
                },
                sel_val
            );
        },
        ()
    );
}

fn pass_delete(app_config: AppConfig) -> RustofiResult {
    return passlist_window(
        &app_config,
        "pass rm",
        |s: &mut String| {
            if s != "" {
                let p_rm = Command::new(GPassCmd::global()).arg("rm").arg(s).stdin(Stdio::piped()).stdout(Stdio::null()).spawn().expect("FAILED TO DELETE");
                writeln!(p_rm.stdin.unwrap(), "y").unwrap();
                println!("Password removed");
            }
            Ok(())
        }
    );
}

fn pass_get(app_config: AppConfig) -> RustofiResult {
    return passlist_window(
        &app_config, 
        "pass >", 
        |s: &mut String| {
            if s != "" {
                Command::new(GPassCmd::global()).arg(s).arg("--clip").stdout(Stdio::null()).spawn().expect("FAILED TO DECRYPT");
                println!("Password copied to clipboard!")
            }
            Ok(())
        }
    );
}

fn passempty_window<T>(app_config: &AppConfig, display: &str, callback: fn(&AppConfig, &str, T) -> RustofiResult, args: T) -> RustofiResult {
    return match EntryBox::display(&app_config.rofi_args, display.to_string()) {
        RustofiResult::Selection(p) => {
            callback(app_config, &p, args)
        },
        _ => RustofiResult::Exit
    };
}

fn passlist_window(app_config: &AppConfig, display: &str, callback: fn(&mut String) -> Result<(), String>) -> RustofiResult {
    let pass_dir: PathBuf = match home_dir() {
        Some(p) => [p, PathBuf::from(app_config.pass_dir.as_str())].iter().collect(),
        _ => return RustofiResult::Exit
    };

    return ItemList::new(
        &app_config.rofi_args,
        traverse_pass_dir(app_config.pass_dir.as_str(), &pass_dir),
        Box::new(callback)).display(display.to_string()
    );
}

fn traverse_pass_dir(root: &str, pass_dir: &PathBuf) -> Vec<String> {
    let mut pass_l: Vec<String> = Vec::new();
    for pass_e in fs::read_dir(pass_dir).unwrap() {
        let pass = pass_e.unwrap().path();
        if pass.is_dir() {
            pass_l.append(&mut traverse_pass_dir(root, &pass));
        } else {
            match pass.extension() {
                Some(s) => if s == "gpg" { pass_l.push(pass.to_str().unwrap().split(root).nth(1).unwrap().replace(".gpg", "").to_string()) },
                _ => {}
            }
        }
    }

    return pass_l;
}
