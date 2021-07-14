mod boilerplate;

use std::path::PathBuf;
use std::{fs, io};
use comrak::{parse_document, format_html, Arena, ComrakOptions};
use comrak::nodes::{AstNode, NodeValue};

fn main() {
    let dest_ext = "html";
    let root = PathBuf::from(SRC_DIR);
    let files = &mut vec![PathBuf::new(); 0];
    find_files(root, SRC_EXT, files).unwrap();
    let mut opts = ComrakOptions::default();
    opts.extension.footnotes = true;
    opts.extension.front_matter_delimiter = Some("---".to_owned());

    for file in files {
        let arena = Arena::new();
        if let Some(root) = parse(&arena, file, &opts) {
            prevent_orphaned_words(root);
            let dest = file;
            dest.set_extension(dest_ext);
            let dest = PathBuf::from(dest.strip_prefix(SRC_DIR).unwrap());
            write(&dest, root, &opts);
        }
    }
}

const SRC_DIR: &str = "src";
const DIST_DIR: &str = "dist";
const SRC_EXT: &str = "md";
const NBSP: &str = "\u{00A0}";

fn parse<'a>(arena: &'a Arena<AstNode<'a>>, path: &PathBuf, opts: &ComrakOptions) -> Option<&'a AstNode<'a>> {
    if let Ok(content) = fs::read(path) {
        return Some(
            parse_document(
                arena,
                String::from_utf8(content).unwrap().as_str(),
                opts)
        );
    }

    None
}

fn iter_nodes<'a, F>(node: &'a AstNode<'a>, f: &F)
    where F : Fn(&'a AstNode<'a>) {
    f(node);
    for c in node.children() {
        iter_nodes(c, f);
    }
}

fn prevent_orphaned_words<'a>(root: &'a AstNode<'a>) {
    iter_nodes(root, &|node| {
        match &mut node.data.borrow_mut().value {
            &mut NodeValue::Text(ref mut text) => {
                let string = String::from_utf8(text.to_vec()).unwrap();
                if let Some(last_space_pos) = string.rfind(r" ") {
                    text.splice(last_space_pos..(last_space_pos + 1), NBSP.bytes());
                }
            }
            _ => (),
        }
    });
}

fn write<'a>(path: &PathBuf, root: &'a AstNode<'a>, opts: &ComrakOptions) {
    let mut html = vec![];
    format_html(root, opts, &mut html).unwrap();
    boilerplate::wrap_body(&mut html);

    let dest_file = PathBuf::from(DIST_DIR).join(path);
    let mut dest_dir = dest_file.clone();
    dest_dir.pop();
    fs::create_dir_all(dest_dir).unwrap();
    fs::write(dest_file, &html).unwrap();
}

fn visit_dirs<'a, F>(path: PathBuf, acc: &'a mut Vec<PathBuf>, predicate: &F) -> Result<(), io::Error>
    where F: Fn(&PathBuf) -> bool {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            visit_dirs(entry.path(), acc, predicate)?;
        }

        return Ok(());
    }

    if predicate(&path) {
        acc.push(path);
    }

    Ok(())
}

fn find_files<'a>(root: PathBuf, extension: &str, files: &'a mut Vec<PathBuf>) -> Result<(), io::Error> {
    let is_markdown = |path: &PathBuf| {
        match path.extension() {
            Some(ext) => ext == extension,
            None => false,
        }
    };
    visit_dirs(root, files, &is_markdown)
}
