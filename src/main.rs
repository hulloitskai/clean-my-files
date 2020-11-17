mod cleanmymac;
mod prelude;

use std::env::set_var as set_env_var;
use std::fs::remove_file;
use std::io::{stdin, Read};
use std::path::Path;

use clap::{AppSettings, Clap};
use cleanmymac::parse_report;
use logger::init as init_logger;
use prelude::*;
use rayon::prelude::*;

#[derive(Debug, Clap)]
#[clap(name = "clean-my-files", version = env!("BUILD_VERSION"))]
#[clap(about = "A little tool to clean up after CleanMyMac X")]
#[clap(setting = AppSettings::ColoredHelp)]
#[clap(setting = AppSettings::DeriveDisplayOrder)]
pub struct Cli {
    #[clap(
        long,
        about = "Log level and directives",
        value_name = "level",
        default_value = "info",
        hide_default_value = true
    )]
    pub log: String,

    #[clap(long, about = "Don't remove any files")]
    pub dry_run: bool,
}

fn main() -> Result<()> {
    // Parse CLI.
    let cli = Cli::parse();

    // Configure logger.
    set_env_var("RUST_LOG", &cli.log);
    init_logger();

    // Read report from input.
    let mut report = String::new();
    stdin().read_to_string(&mut report).context("read input")?;

    // Parse report for clean errors.
    let errors = parse_report(&report).context("parse report")?;
    let paths: Vec<_> = errors.iter().map(|error| &error.file_path).collect();
    let removals = remove_all(&paths, cli.dry_run);
    let failures = paths.len() - removals;

    // Report summary.
    if removals > 0 && failures > 0 {
        info!("removed {} files, failed on {} files", removals, failures);
    } else if removals > 0 {
        info!("removed all {} files", removals);
    } else if failures > 0 {
        info!("failed on all {} files", failures);
    }

    Ok(())
}

pub fn remove_all(paths: &[impl AsRef<Path> + Sync], feign: bool) -> usize {
    let removals: Vec<()> = paths
        .par_iter()
        .filter_map(|path| {
            let path = path.as_ref();
            if !path.exists() {
                warn!("'{}' does not exist (skipped)", path.display());
                return None;
            }
            if !feign {
                if let Err(error) = remove_file(path) {
                    error!("failed to remove '{}': {}", path.display(), &error);
                    return None;
                };
            }
            info!("removed {}", path.display());
            Some(())
        })
        .collect();
    removals.len()
}
