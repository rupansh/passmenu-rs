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
mod widgets;

use crate::widgets::*;

use config::AppConfig;
use once_cell::sync::OnceCell;
use rustofi::RustofiResult;
use std::env;
use std::io::Write;
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


fn main() -> Result<(), String> {
    let app_config = config::get_conf();
    if let Err((e, mut args)) = app_config {
        return err_info(&mut args, e);
    }
    let mut app_config = app_config.unwrap();

    if let RustofiResult::Error(e) = app_main(&mut app_config) {
        return err_info(&mut app_config.rofi_args, e);
    }

    Ok(())
}

fn app_main(app_config: &mut AppConfig) -> RustofiResult {
    let args = env::args().collect::<Vec<String>>();

    APASS_CMD.set(app_config.pass_cmd.clone()).unwrap();

    let arg_iter = args.iter().map(String::as_str);
    for arg in arg_iter.clone() {
        // TODO: Better way to do this
        if arg == "new" || arg == "insert" {
            zero_lines(app_config);
        }

        match arg {
            "new" => return pass_generate(app_config),
            "insert" => return pass_insert(app_config),
            "del" => return pass_delete(app_config),
            "otp" => return otp::parse_cmd(app_config, arg_iter.clone()),
            _ => ()
        }
    }

    return pass_get(app_config)
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
