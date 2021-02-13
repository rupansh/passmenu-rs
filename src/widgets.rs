use crate::{
    config::AppConfig,
    utils::traverse_pass_dir
};

use dirs::home_dir;
use rustofi::{
    components::*,
    RustofiResult
};
use std::path::PathBuf;

pub fn passempty_window<T>(app_config: &AppConfig, display: &str, callback: fn(&AppConfig, &str, T) -> RustofiResult, args: T) -> RustofiResult {
    return match EntryBox::display(&app_config.rofi_args, display.to_string()) {
        RustofiResult::Selection(p) => {
            callback(app_config, &p, args)
        },
        e => e
    };
}

pub fn passlist_window(app_config: &AppConfig, display: &str, callback: fn(&mut String) -> Result<(), String>) -> RustofiResult {
    let pass_dir: PathBuf = [home_dir().unwrap(), PathBuf::from(app_config.pass_dir.as_str())].iter().collect();

    return ItemList::new(
        &app_config.rofi_args,
        traverse_pass_dir(app_config.pass_dir.as_str(), &pass_dir),
        Box::new(callback)
    ).display(display.to_string());
}
