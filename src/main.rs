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
mod utils;
mod otp;

use config::AppConfig;
use dirs::home_dir;
use once_cell::sync::OnceCell;
use rustofi::{
    components::*,
    RustofiResult,
};
use std::env;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use utils::*;

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
    let mut res = app_main();
    match res.0 {
        RustofiResult::Error(e) => err_info(&mut res.1, e),
        _ => return
    }
}

fn app_main() -> (RustofiResult, Vec<String>) {
    let app_config = config::get_conf();
    let args = env::args().collect::<Vec<String>>();

    if app_config.is_err() {
        let err = app_config.err().unwrap();
        return (RustofiResult::Error(err.0), err.1)
    }

    let mut app_config = app_config.unwrap();
    APASS_CMD.set(app_config.pass_cmd.clone()).unwrap();

    let arg_iter = args.iter().map(String::as_str);
    for arg in arg_iter.clone() {
        // TODO: Better way to do this
        if arg == "new" || arg == "ins" {
            zero_lines(&mut app_config);
        }

        match arg {
            "new" => return (pass_generate(&app_config), app_config.rofi_args),
            "ins" => return (pass_insert(&app_config), app_config.rofi_args),
            "del" => return (pass_delete(&app_config), app_config.rofi_args),
            "otp" => return (otp::parse_cmd(&mut app_config, arg_iter.clone()), app_config.rofi_args),
            _ => ()
        }
    }

    return (pass_get(&app_config), app_config.rofi_args)
}

fn pass_generate(app_config: &AppConfig) -> RustofiResult {
    return passempty_window(
        &app_config,
        "pass generate",
        |app_config: &AppConfig, sel_val: &str, ()| -> RustofiResult {
            if Command::new(app_config.pass_cmd.as_str())
                .arg("generate")
                .arg("--clip")
                .arg(sel_val)
                .stdout(Stdio::null())
                .spawn()
                .is_err() {
                    return RustofiResult::Error("Failed to run pass generate".to_string())
                }

            println!("Password Generated and copied to clipboard!");
            return RustofiResult::Success;
        },
        ()
    );
}

fn pass_insert(app_config: &AppConfig) -> RustofiResult {
    return passempty_window(
        &app_config,
        "pass insert",
        |app_config: &AppConfig, sel_val: &str, ()| -> RustofiResult {
            return passempty_window(
                app_config,
                "Enter password to insert",
                |app_config: &AppConfig, sel_val: &str, prev_val: &str| -> RustofiResult {
                    let p_ins = 
                    Command::new(app_config.pass_cmd.as_str())
                        .arg("insert").arg(prev_val)
                        .stdin(Stdio::piped())
                        .stdout(Stdio::null())
                        .spawn();

                    if p_ins.is_err() {
                        return RustofiResult::Error("Failed to run pass insert".to_string());
                    }

                    let p_ins = p_ins.unwrap();
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

fn pass_delete(app_config: &AppConfig) -> RustofiResult {
    return passlist_window(
        &app_config,
        "pass rm",
        |s: &mut String| {
            if s != "" {
                let p_rm = 
                Command::new(GPassCmd::global())
                    .arg("rm")
                    .arg(s)
                    .stdin(Stdio::piped())
                    .stdout(Stdio::null())
                    .spawn();

                if p_rm.is_err() {
                    return Err("Failed to run pass rm".to_string());
                }

                let p_rm = p_rm.unwrap();
                writeln!(p_rm.stdin.unwrap(), "y").unwrap();
                println!("Password removed");
            }
            Ok(())
        }
    );
}

fn pass_get(app_config: &AppConfig) -> RustofiResult {
    return passlist_window(
        &app_config, 
        "pass >", 
        |s: &mut String| {
            if s != "" {
                if Command::new(GPassCmd::global())
                    .arg(s)
                    .arg("--clip")
                    .stdout(Stdio::null())
                    .spawn()
                    .is_err() {
                        return Err("Failed to run pass".to_string());
                    }

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
        e => e
    };
}

fn passlist_window(app_config: &AppConfig, display: &str, callback: fn(&mut String) -> Result<(), String>) -> RustofiResult {
    let pass_dir: PathBuf = [home_dir().unwrap(), PathBuf::from(app_config.pass_dir.as_str())].iter().collect();

    return ItemList::new(
        &app_config.rofi_args,
        traverse_pass_dir(app_config.pass_dir.as_str(), &pass_dir),
        Box::new(callback)).display(display.to_string()
    );
}
