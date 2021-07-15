use std::{fs, io, str};
use std::path::PathBuf;

pub fn find_files<'a>(root: &str, extension: &str) -> Result<Vec<PathBuf>, io::Error> {
    let mut files = vec![PathBuf::new(); 0];

    let is_markdown = |path: &PathBuf| {
        match path.extension() {
            Some(ext) => ext == extension,
            None => false,
        }
    };

    visit_dirs(PathBuf::from(root), &mut files, &is_markdown)?;

    Ok(files.to_owned())
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
