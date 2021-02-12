use crate::config::AppConfig;
use crate::{passempty_window, passlist_window, GPassCmd, GetGlobal};
use crate::utils::zero_lines;

use rustofi::RustofiResult;
use std::io::Write;
use std::process::{Command, Stdio};


pub fn parse_cmd<'a>(app_config: &mut AppConfig, cmd_list: impl Iterator<Item = &'a str>) -> RustofiResult {
    for cmd in cmd_list {
        match cmd {
            "insert" => { zero_lines(app_config); return pass_otp_insert(app_config) },
            _ => ()
        }
    }
    return pass_otp(app_config);
}


fn pass_otp(app_config: &AppConfig) -> RustofiResult {
    return passlist_window(&app_config, "pass otp >", |s: &mut String| {
        if s != "" {
            if Command::new(GPassCmd::global())
                .arg("otp")
                .arg(s)
                .arg("--clip")
                .stdout(Stdio::null())
                .spawn()
                .is_err()
            {
                return Err("Failed to run pass".to_string());
            }

            println!("Password copied to clipboard!")
        }
        Ok(())
    });
}

fn pass_otp_insert(app_config: &AppConfig) -> RustofiResult {
    return passempty_window(
        &app_config,
        "pass otp insert",
        |app_config: &AppConfig, sel_val: &str, ()| -> RustofiResult {
            return passempty_window(
                app_config,
                "Enter OTP to insert",
                |app_config: &AppConfig, sel_val: &str, prev_val: &str| -> RustofiResult {
                    let p_ins = Command::new(app_config.pass_cmd.as_str())
                        .arg("otp")
                        .arg("insert")
                        .arg(prev_val)
                        .stdin(Stdio::piped())
                        .stdout(Stdio::null())
                        .spawn();

                    if p_ins.is_err() {
                        return RustofiResult::Error("Failed to run pass otp insert".to_string());
                    }

                    let p_ins = p_ins.unwrap();
                    writeln!(p_ins.stdin.unwrap(), "{}\n{}", sel_val, sel_val).unwrap();
                    println!("Pass OTP added!");
                    return RustofiResult::Success;
                },
                sel_val,
            );
        },
        (),
    );
}
