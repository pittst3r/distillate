use std::{str, fs, path::PathBuf};
use chrono::{Utc, Datelike};
use handlebars::{Handlebars, RenderError};
use serde_json::json;
use crate::{Config, State};

pub fn wrap(body: &mut String, conf: &Config, state: &State) -> Result<(), RenderError> {
    let year = Utc::now().date().year();
    let values = json!({
        "page": {
            "body": body,
            "is-home": state.is_home,
            "title": state.page_title,
        },
        "site": {
            "title": conf.title,
            "copyright": conf.copyright,
            "year": year,
        },
    });
    let hbs = Handlebars::new();
    let template_path = PathBuf::from(&conf.template_path);
    let content = fs::read(&template_path).unwrap();
    let content = str::from_utf8(&content).unwrap();

    *body = hbs.render_template(content, &values)?;

    Ok(())
}
