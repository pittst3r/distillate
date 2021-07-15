mod markdown_to_html;
mod boilerplate;
mod files;

use std::{str};
use markdown_to_html::markdown_to_html;
use files::find_files;
use structopt::StructOpt;

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

fn main() {
    let args = Opt::from_args();

    match args.cmd {
        Some(Command::Build) => {
            match markdown_to_html(SRC_DIR, SRC_EXT, DIST_DIR) {
                Ok(_) => (),
                Err(e) => println!("{}", e),
            };
        },
        _ => Opt::clap().print_help().unwrap(),
    }
}
