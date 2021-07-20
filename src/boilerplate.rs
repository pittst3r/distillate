use chrono::{Utc, Datelike};
use handlebars::{Handlebars, RenderError};
use serde_json::json;
use crate::{Config, typography};

const TEMPLATE: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width">
<title>{{title}}</title>
</head>

<body>
<h1><a href="/" title="Go to homepage">{{heading}}</a></h1>
{{{body}}}
<hr />
<p>© {{year}} {{copyright}}</p>
</body>

</html>
"#;

pub fn wrap(body: &mut String, conf: &Config) -> Result<(), RenderError> {
    let year = Utc::now().date().year();
    let heading = typography::replace_last_bsp_with_nbsp(&conf.heading);
    let hbs = Handlebars::new();
    let values = json!({
        "title": conf.title,
        "heading": heading,
        "copyright": conf.copyright,
        "year": year,
        "body": body
    });

    *body = hbs.render_template(TEMPLATE, &values)?;

    Ok(())
}
