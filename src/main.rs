mod app;
mod config;
mod opt;
mod runner;
mod util;

use config::Config;
use opt::Opt;
use structopt::StructOpt;

fn main() {
    let config =
        confy::load::<Config>(&app::app_name(), "config").expect("failed to read config file");
    let opt = Opt::from_args();

    if opt.debug {
        eprintln!("config:\n{config:#?}\n");
        eprintln!("opt:\n{opt:#?}\n");
    }

    runner::run_command(&config, &opt).unwrap();
}
