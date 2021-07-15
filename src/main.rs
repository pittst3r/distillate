mod boilerplate;

use std::path::PathBuf;
use std::{fs, io, str};
use comrak::{parse_document, format_html, Arena, ComrakOptions};
use comrak::nodes::{AstNode, NodeValue};

const SRC_DIR: &str = "src";
const DIST_DIR: &str = "dist";
const SRC_EXT: &str = "md";
const DEST_EXT: &str = "html";
const NBSP: &str = "\u{00A0}";

fn main() {
    let root = PathBuf::from(SRC_DIR);
    let files = match find_files(root, SRC_EXT) {
        Ok(files) => files,
        Err(e) => return println!("Came into difficulty finding files to process: {}", e),
    };

    let arena = Arena::new();

    for file in files {
        if let Ok(md) = MarkdownFile::new(&arena, &file) {
            md.prevent_orphaned_words();

            let html = match HTMLFile::try_from(md) {
                Ok(html) => html,
                Err(e) => return println!("Failed to generate HTML from file '{}': {}", file.to_str().unwrap_or("None"), e),
            };

            let dest = src_path_to_dest_path(file, DEST_EXT);

            if let Err(e) = html.persist(dest) {
                println!("Failed to save generated file: {}", e)
            }
        }
    }
}

struct MarkdownFile<'a> {
    inner: &'a AstNode<'a>,
}

struct HTMLFile {
    inner: Vec<u8>,
}

trait Persistable {
    type Error;
    
    fn persist(&self, dest: PathBuf) -> Result<(), Self::Error>;
}

impl Persistable for HTMLFile {
    type Error = io::Error;

    fn persist(&self, dest: PathBuf) -> Result<(), Self::Error> {
        let dest_file = PathBuf::from(DIST_DIR).join(dest);
        fs::create_dir_all(dest_file.parent().unwrap())?;
        fs::write(dest_file, &self.inner)?;

        Ok(())
    }
}

impl<'a> MarkdownFile<'a> {
    fn new(arena: &'a Arena<AstNode<'a>>, path: &PathBuf) -> Result<MarkdownFile<'a>, String> {
        let path_str = path.to_str().unwrap_or("None");

        if let Ok(content) = fs::read(path) {
            let opts = comrak_options();
            if let Ok(content) = str::from_utf8(&content[..]) {
                let inner = parse_document(arena, content, &opts);
                return Ok(MarkdownFile { inner });
            } else {
                return Err(format!("Unable to parse file as utf8 at path '{}'", path_str));
            }
        }

        Err(format!("Unable to parse markdown file at path '{}'", path_str))
    }

    fn iter_nodes<F>(node: &'a AstNode<'a>, f: &F)
        where F : Fn(&'a AstNode<'a>) {
        f(node);
        for c in node.children() {
            Self::iter_nodes(c, f);
        }
    }

    fn prevent_orphaned_words(&self) {
        let nodes: &'a AstNode<'a> = &self.inner;
        Self::iter_nodes(nodes, &|node| {
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
}

// TODO
// impl<'a> std::convert::TryFrom<MarkdownFile<'a>> for HTMLFile {
impl HTMLFile {
    fn try_from(origin: MarkdownFile) -> Result<Self, io::Error> {
        let mut html = vec![];
        let opts = comrak_options();
        format_html(origin.inner, &opts, &mut html)?;
        boilerplate::wrap_body(&mut html);

        Ok(HTMLFile { inner: html })
    }
}

fn comrak_options() -> ComrakOptions {
    let mut opts = ComrakOptions::default();
    opts.extension.footnotes = true;
    opts.extension.front_matter_delimiter = Some("---".to_owned());
    opts
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

fn find_files<'a>(root: PathBuf, extension: &str) -> Result<Vec<PathBuf>, io::Error> {
    let files = &mut vec![PathBuf::new(); 0];

    let is_markdown = |path: &PathBuf| {
        match path.extension() {
            Some(ext) => ext == extension,
            None => false,
        }
    };

    visit_dirs(root, files, &is_markdown)?;

    Ok(files.to_owned())
}

fn src_path_to_dest_path(mut file: PathBuf, dest_ext: &str) -> PathBuf {
    file.set_extension(dest_ext);
    file = PathBuf::from(file.strip_prefix(SRC_DIR).unwrap());
    file
}
