use std::{fs, io, str};
use std::path::PathBuf;
use comrak::{parse_document, format_html, Arena, ComrakOptions};
use comrak::nodes::{AstNode, NodeValue};
use crate::boilerplate;
use crate::find_files;

const DIST_EXT: &str = "html";
const NBSP: &str = "\u{00A0}";

pub fn markdown_to_html(src_dir: &str, src_ext: &str, dist_dir: &str) -> Result<(), String> {
    let files = match find_files(src_dir, src_ext) {
        Ok(files) => files,
        Err(e) => return Err(format!("Came into difficulty finding files to process: {}", e)),
    };
    let arena = Arena::new();

    for file in files {
        if let Ok(md) = MarkdownFile::new(&arena, &file) {
            md.prevent_orphaned_words();

            let html = match HTMLFile::try_from(md) {
                Ok(h) => h,
                Err(e) => {
                    return Err(format!("Unable to convert markdown to html for file '{}': {}", file.clone().to_str().unwrap_or("???"), e))
                }
            };

            let dest = src_path_to_dest_path(file, src_dir, dist_dir, DIST_EXT);

            match html.persist(&dest) {
                Ok(()) => (),
                Err(e) => return Err(format!("Unable to persist file '{}': {}", dest.to_str().unwrap_or("???"), e)),
            }
        }
    }

    Ok(())
}

struct MarkdownFile<'a> {
    inner: &'a AstNode<'a>,
}

struct HTMLFile {
    inner: Vec<u8>,
}

trait Persistable {
    type Error;
    
    fn persist(&self, dest: &PathBuf) -> Result<(), Self::Error>;
}

impl Persistable for HTMLFile {
    type Error = io::Error;

    fn persist(&self, dest: &PathBuf) -> Result<(), Self::Error> {
        fs::create_dir_all(dest.parent().unwrap())?;
        fs::write(dest, &self.inner)?;

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

fn src_path_to_dest_path(file: PathBuf, src_dir: &str, dist_dir: &str, dest_ext: &str) -> PathBuf {
    let file = file.strip_prefix(src_dir).unwrap().with_extension(dest_ext);
    
    PathBuf::from(dist_dir).join(file)
}
