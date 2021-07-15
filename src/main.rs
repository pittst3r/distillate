mod markdown_to_html;
mod boilerplate;
mod files;

use std::{str};
use markdown_to_html::markdown_to_html;
use files::find_files;

const SRC_DIR: &str = "src";
const SRC_EXT: &str = "md";
const DIST_DIR: &str = "dist";

fn main() {
    match markdown_to_html(SRC_DIR, SRC_EXT, DIST_DIR) {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    };
}
