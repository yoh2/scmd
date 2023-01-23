mod app;
mod config;
mod opt;
mod runner;
mod util;

use config::Config;
use opt::Opt;
use runner::Error;
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

    if let Err(e) = runner::run_command(&config, &opt) {
        if let Error::ExecvpeFailed { file, errno } = e {
            eprintln!("{}: {}", file, strerror(errno));
        } else {
            eprintln!("{e}");
        }
    }
}
