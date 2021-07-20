use std::{fs, io, str};
use std::path::PathBuf;
use pulldown_cmark::{Event, CowStr, Parser, Options, html};
use crate::{Config, boilerplate, find_files, typography};

const DIST_EXT: &str = "html";

pub fn markdown_to_html(src_dir: &str, src_ext: &str, dist_dir: &str, conf: &Config) -> Result<(), String> {
    let files = match find_files(src_dir, src_ext) {
        Ok(files) => files,
        Err(e) => return Err(format!("Errored while finding files to process: {}", e)),
    };
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    for file in files {
        let content = fs::read(&file).unwrap();
        let content = str::from_utf8(content.as_slice()).unwrap();
        let parser = Parser::new_ext(content, options);
        let events = prevent_orphaned_words(parser);
        let dest = src_path_to_dest_path(&file, src_dir, dist_dir, DIST_EXT);
        let mut content = String::new();

        html::push_html(&mut content, events);
        
        match boilerplate::wrap(&mut content, conf) {
            Err(e) => return Err(format!("Unable to generate html from template '{}': {}", file.to_str().unwrap_or("???"), e)),
            _ => (),
        };

        if let Err(e) = persist(content, &dest) {
            return Err(format!("Unable to save file to '{}': {}", dest.to_str().unwrap_or("???"), e));
        }
    }

    Ok(())
}

fn prevent_orphaned_words(parser: Parser) -> impl Iterator<Item=Event> {
    parser.map(move |event| match event {
        Event::Text(text) => Event::Text(CowStr::from(typography::replace_last_bsp_with_nbsp(&text.into_string()))),
        _ => event,
    })
}

fn src_path_to_dest_path(file: &PathBuf, src_dir: &str, dist_dir: &str, dest_ext: &str) -> PathBuf {
    let file = file.strip_prefix(src_dir).unwrap().with_extension(dest_ext);
    
    PathBuf::from(dist_dir).join(file)
}

fn persist(content: String, dest: &PathBuf) -> io::Result<()> {
    fs::create_dir_all(dest.parent().unwrap())?;
    fs::write(dest, content)
}
