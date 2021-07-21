mod markdown_to_html;
mod template;
mod typography;
mod files;

use std::{str, fs, convert::{TryFrom}};
use markdown_to_html::markdown_to_html;
use files::find_files;
use structopt::StructOpt;
use toml::Value;
use handlebars::JsonValue;

const SRC_DIR: &str = "src";
const SRC_EXT: &str = "md";

#[derive(StructOpt)]
#[structopt(name = "distillate", about = "Markdown to HTML static site generator")]
struct Opt {
    #[structopt(subcommand)]
    cmd: Option<Command>
}

#[derive(StructOpt)]
enum Command {
    Build {
        dest: String
    }
}

pub struct Config {
    title: String,
    copyright: String,
    template_path: String,
}

pub struct State {
    is_home: JsonValue,
    page_title: JsonValue,
}

fn main() {
    let args = Opt::from_args();

    match args.cmd {
        Some(Command::Build { dest }) => {
            let conf = match load_config() {
                Ok(conf) => conf,
                Err(e) => return println!("{}", e),
            };

            match markdown_to_html(SRC_DIR, SRC_EXT, dest.as_str(), &conf) {
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
            copyright: unwrap_config_value!(map, "copyright"),
            template_path: map["template-path"].as_str().unwrap().to_string()
        })
    }
}
