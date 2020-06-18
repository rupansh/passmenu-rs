# passmenu-rs

[![License: GPL v3](https://img.shields.io/badge/license-GPL%20v3-blue?style=for-the-badge&logo=GNU)](https://www.gnu.org/licenses/gpl-3.0)
[![RUST](https://img.shields.io/badge/made%20with-RUST-red.svg?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)

![screen](https://github.com/rupansh/passmenu-rs/blob/master/ss.png?raw=true)

*Quick and dirty rofi frontend for [passwordstore](https://www.passwordstore.org/)*

YES I READ ROFI's README.MD's LAST LINE

**BETA SOFTWARE**
#


### Prerequisites
Rust 2018 \
passwordstore (`pass`) \
[rofi](https://github.com/davatorium/rofi/)

### Usage
`cargo install --git https://github.com/rupansh/passmenu-rs.git --branch master`

`passmenu-rs [optional-command]`

### Available Commands
- `new` - Generate a new password
- `del` - Delete a password
- `insert` - Insert a password

### Config
example `~/.config/passmenu_rs` (TOML format)
```
rofi_args = "-dpi 80 -show-icons -theme /home/rupansh/slate -lines 8 -padding 18 -width 120 -location 0 -sidebar-mode"
```

### Todo
- General GUI improvements
- Implement other pass related commands
