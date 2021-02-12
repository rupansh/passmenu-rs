use crate::AppConfig;

use rustofi::components::EntryBox;
use std::fs;
use std::path::PathBuf;

pub fn err_info(rofi_args: &mut Vec<String>, s: String) {
    match rofi_args.iter().position(|r| r == "-lines") {
        Some(i) => rofi_args[i+1] = "0".to_string(),
        _ => { 
            rofi_args.push("-lines".to_string());
            rofi_args.push("0".to_string())
        }
    }

    EntryBox::display(&rofi_args, format!("passmenu-rs failed with err: {}", s));
}

pub fn traverse_pass_dir(root: &str, pass_dir: &PathBuf) -> Vec<String> {
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

pub fn zero_lines(app_config: &mut AppConfig) {
    match app_config.rofi_args.iter().position(|r| r == "-lines") {
        Some(i) => app_config.rofi_args[i+1] = "0".to_string(),
        _ => { 
            app_config.rofi_args.push("-lines".to_string());
            app_config.rofi_args.push("0".to_string())
        }
    }
}
