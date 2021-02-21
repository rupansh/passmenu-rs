# passmenu-rs

[![License: GPL v3](https://img.shields.io/badge/license-GPL%20v3-blue?style=for-the-badge&logo=GNU)](https://www.gnu.org/licenses/gpl-3.0)
[![RUST](https://img.shields.io/badge/made%20with-RUST-red.svg?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)

![screen](https://github.com/rupansh/passmenu-rs/blob/master/ss.png?raw=true)

*Quick and dirty rofi frontend for [passwordstore](https://www.passwordstore.org/)*

YES I READ ROFI's README.MD's LAST LINE

**BETA SOFTWARE**
#


# Installation

### Cargo

#### Prerequisites
Rust 2018 \
passwordstore (`pass`) \
[rofi](https://github.com/davatorium/rofi/)

`cargo install --git https://github.com/rupansh/passmenu-rs.git --branch master`

### Arch Linux (AUR)
[passmenu-rs-git](https://aur.archlinux.org/packages/passmenu-rs-git/)

## Usage
`passmenu-rs [optional-command]`

## Available Commands
- `new` - Generate a new password
- `del` - Delete a password
- `insert` - Insert a password

## Available Subcommands

### [Pass OTP](https://github.com/tadfisher/pass-otp)
- `otp` - Get OTP passcodes
- `otp insert` - Insert OTP key as a url

## Config
Available Options
- `rofi_args` - arguments for rofi
- `pass_cmd` - optional password store command
- `pass_dir` - optional password store directory from `~/`

example `~/.config/passmenu_rs` (TOML format)
```
rofi_args = "-dpi 80 -show-icons -theme /home/rupansh/slate -lines 8 -padding 18 -width 120 -location 0 -sidebar-mode"
```

### Todo
- General GUI improvements
- Implement other pass related commands
