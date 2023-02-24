mod app;
mod config;
mod opt;
mod runner;
mod util;

use crate::config::{parameter::ParamConfigs, CommandConfigs};
use config::Config;
use itertools::Itertools;
use opt::Opt;
use runner::Error;
use std::{collections::BTreeMap, process::exit};
use structopt::StructOpt;
use util::libc::{set_all_locale_by_env, strerror};

fn main() {
    set_all_locale_by_env();

    let config =
        confy::load::<Config>(&app::app_name(), "config").expect("failed to read config file");
    let opt = Opt::from_args();

    if opt.debug {
        eprintln!("config:\n{config:#?}\n");
        eprintln!("opt:\n{opt:#?}\n");
    }

    if opt.list {
        if let Some(command) = &opt.command {
            print_command_detail_then_exit(&config, command);
        } else {
            list_commands_then_exit(&config.command);
        }
    }

    if let Err(e) = runner::run_command(&config, &opt) {
        if let Error::ExecvpeFailed { file, errno } = e {
            eprintln!("{}: {}", file, strerror(errno));
        } else {
            eprintln!("{e}");
        }
    }
}

fn print_command_detail_then_exit(config: &Config, command: &str) -> ! {
    let Some(command_config) = config.command.get(command) else {
        eprintln!("command `{command}` is not defined");
        exit(1);
    };

    println!("command `{command}`:");
    println!("  base: {}", quote_strings(command_config.base.as_slice()));
    let placeholder = command_config
        .placeholder
        .as_ref()
        .unwrap_or(&config.default.placeholder);
    println!("  placeholder: {placeholder}",);

    println!("  headparams:");
    list_param_configs("    ", &command_config.headparams);

    println!("  middleparams:");
    list_param_configs("    ", &command_config.middleparams);

    println!("  tailparams:");
    list_param_configs("    ", &command_config.tailparams);

    exit(0);
}

fn list_param_configs(prefix: &str, param_configs: &ParamConfigs) {
    if param_configs.is_empty() {
        println!("{prefix}<n/a>");
        return;
    }
    let max_name_len = max_key_len(param_configs);
    for (name, def) in param_configs.iter() {
        println!(
            "{prefix}{name:max_name_len$} : {}",
            quote_strings(def.strings())
        )
    }
}

fn list_commands_then_exit(configs: &CommandConfigs) -> ! {
    println!("defined commands:");
    if configs.is_empty() {
        println!("  <n/a>");
    } else {
        let max_command_len = max_key_len(configs);
        for (command, config) in configs.iter() {
            println!(
                "  {command:max_command_len$} : {}",
                quote_strings(config.base.as_slice())
            )
        }
    }
    exit(0);
}

fn quote_strings(strings: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    strings
        .into_iter()
        .map(|s| format!(r#""{}""#, s.as_ref().escape_debug()))
        .join(" ")
}

fn max_key_len<T>(map: &BTreeMap<String, T>) -> usize {
    map.iter()
        .map(|(command, _)| command.len())
        .max()
        .unwrap_or(0)
}
