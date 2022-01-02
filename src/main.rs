#![feature(bool_to_option)]

mod appconfig;
mod cli;
pub mod error;

use crate::appconfig::AppConfig;
use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use error::AppError;
use regex::Regex;
use sysinfo::{Process, ProcessExt, System, SystemExt};

fn main() -> Result<()> {
    let conf = AppConfig::new()?;
    let cmd = Cli::parse();
    let mut sys = System::new();

    match &cmd.command {
        Commands::Process { item } => process(&mut sys, &conf, item),
    }
}

fn process(sys: &mut System, conf: &AppConfig, item: &String) -> Result<()> {
    println!("Process matching {} pattern will be killed.", item);
    let matches = conf
        .process
        .get(item)
        .ok_or(AppError::ItemNotFound(item.into()))?;
    println!("Item content is: {}", matches);

    let re = Regex::new(&matches).map_err(AppError::Pattern)?;

    sys.refresh_processes();
    let current_pid =
        sysinfo::get_current_pid().map_err(|e| AppError::CurrentPid(e.to_string()))?;

    let procs: Vec<&Process> = sys
        .processes()
        .into_iter()
        // Filter matching processes except this one
        .filter(|proc| re.is_match(&proc.1.cmd().join(" ")) && &current_pid != proc.0)
        .map(|proc| proc.1)
        .collect();

    procs.iter().for_each(|proc| {
        let cmd = &proc.cmd().join(" ");
        let ok = &proc.kill();
        println!("{} -- {}", ok, cmd);
    });

    Ok(())
}
