mod config;
mod utils;

use new_string_template::template::Template;

use std::collections::HashMap;

pub use std::io::{self, Write};

use crate::config::{Config, SubConfig};
use crate::utils::speed_test;
use clap::{Args, Parser, Subcommand};

use indicatif::{ProgressBar, ProgressStyle};
use lazy_static::lazy_static;
use std::ffi::OsString;
use std::path::PathBuf;
use std::process::Command;

lazy_static! {
    static ref CONFIG: Config = serde_json::from_str(include_str!("./config.json")).unwrap();
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 配置 python pypi
    Pypi(MirrorArgs),
    /// 配置 nodejs npm
    Npm(MirrorArgs),

    #[command(external_subcommand)]
    External(Vec<OsString>),
}

#[derive(Debug, Args)]
struct MirrorArgs {
    #[command(subcommand)]
    command: Option<MirrorCommands>,
}

#[derive(Debug, Subcommand)]
#[command(args_conflicts_with_subcommands = true)]
enum MirrorCommands {
    /// Test mirror speed
    Test {
        /// Set a timeout for downloading test file
        #[arg(short, long)]
        timeout: Option<u32>,
    },
    /// List all mirrors
    List {},
    /// Config a mirror
    Conf {
        /// Mirror name
        name: String,
    },
}

fn sub_command(cmd: MirrorCommands, conf: &SubConfig) {
    match cmd {
        MirrorCommands::List {} => {
            println!("display available mirrors:");
            for mirror in &conf.mirrors {
                println!("{}({})", mirror.name, mirror.label);
            }
        }
        MirrorCommands::Test { timeout } => {
            println!("start test pypi mirror speed...");
            for mirror in &conf.mirrors {
                let timeout = timeout.unwrap_or(5);
                let _ = speed_test(mirror, timeout);
            }
        }
        MirrorCommands::Conf { name } => {
            for mirror in &conf.mirrors {
                if mirror.name != name {
                    continue;
                }

                let templ = Template::new(conf.cmd.clone());
                let data = {
                    let mut map = HashMap::new();
                    map.insert("0", mirror.args.clone().unwrap());
                    map
                };

                let command: String = templ.render(&data).unwrap();
                println!("Exec: {}", command);

                let res = if cfg!(target_os = "windows") {
                    Command::new("cmd")
                        .args(["/C", &command])
                        .output()
                        .expect("failed to execute process")
                } else {
                    Command::new("sh")
                        .arg("-c")
                        .arg(command)
                        .output()
                        .expect("failed to execute process")
                };
                if let Ok(stdout) = String::from_utf8(res.stdout) {
                    if !stdout.is_empty() {
                        println!("{}", stdout.trim_end());
                    }
                }
                if let Ok(stderr) = String::from_utf8(res.stderr) {
                    if !stderr.is_empty() {
                        println!("{}", stderr.trim_end());
                    }
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // println!("{:?}", *CONFIG);
    let cli = Cli::parse();

    match cli.command {
        Commands::Pypi(sub) => {
            let command = sub.command.unwrap();
            let conf = CONFIG.pypi.as_ref().unwrap();
            sub_command(command, conf);
        }
        Commands::Npm(sub) => {
            let command = sub.command.unwrap();
            let conf = CONFIG.npm.as_ref().unwrap();
            sub_command(command, conf);
        }
        Commands::External(args) => {
            println!("Calling out to {:?} with {:?}", &args[0], &args[1..]);
        }
    }

    Ok(())
}
