use crate::config::AppConfig;
use crate::{passlist_window, GPassCmd, GetGlobal};

use rustofi::RustofiResult;
use std::process::{Command, Stdio};

pub fn pass_otp(app_config: &AppConfig) -> RustofiResult {
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
