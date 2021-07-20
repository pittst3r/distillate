mod markdown_to_html;
mod boilerplate;
mod typography;
mod files;

use std::{str, fs, convert::{TryFrom}};
use markdown_to_html::markdown_to_html;
use files::find_files;
use structopt::StructOpt;
use toml::Value;

const SRC_DIR: &str = "src";
const SRC_EXT: &str = "md";
const DIST_DIR: &str = "dist";

#[derive(StructOpt)]
#[structopt(name = "distillate", about = "Markdown to HTML static site generator")]
struct Opt {
    #[structopt(subcommand)]
    cmd: Option<Command>
}

#[derive(StructOpt)]
enum Command {
    Build
}

pub struct Config {
    title: String,
    heading: String,
    copyright: String,
}

fn main() {
    let args = Opt::from_args();

    match args.cmd {
        Some(Command::Build) => {
            let conf = match load_config() {
                Ok(conf) => conf,
                Err(e) => return println!("{}", e),
            };

            match markdown_to_html(SRC_DIR, SRC_EXT, DIST_DIR, &conf) {
                Ok(_) => (),
                Err(e) => return println!("{}", e),
            };
        },
        _ => Opt::clap().print_help().unwrap(),
    }
}

fn load_config() -> Result<Config, String> {
    let content = match fs::read("distillate.toml") {
        Ok(content) => content,
        Err(e) => return Err(format!("Unable to read './distillate.toml' file: {}", e)),
    };
    let content = match String::from_utf8(content) {
        Ok(content) => content,
        Err(e) => return Err(format!("Unable to read './distillate.toml' as utf8: {}", e)),
    };
    let cfg = match content.parse::<Value>() {
        Ok(cfg) => cfg,
        Err(e) => return Err(format!("Unable to parse './distillate.toml' as toml: {}", e)),
    };

    Config::try_from(cfg)
}

macro_rules! unwrap_config_value {
    ($map:expr, $key:expr) => {
        match $map[$key].as_str() {
            Some(val) => String::from(val),
            None => return Err(format!("Unable to parse the config value for '{}' in the './distillate.toml' file", $key)),
        }
    }
}

impl TryFrom<Value> for Config {
    type Error = String;

    fn try_from(map: Value) -> Result<Self, Self::Error> {
        Ok(Config {
            title: unwrap_config_value!(map, "title"),
            heading: unwrap_config_value!(map, "heading"),
            copyright: unwrap_config_value!(map, "copyright"),
        })
    }
}
