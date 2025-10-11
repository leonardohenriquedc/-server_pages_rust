use std::{env, fs};

use askama::Template;

#[derive(Template)]
#[template(path = "layout/index.html")]
pub struct TemplateIndex {
    pub style_sheet: String,
    links: Vec<String>,
    link_request_about: String,
}

impl TemplateIndex {
    pub fn insert_tag_ancora(&mut self, link: String, content: String) {
        let tag_ancora = format!("\n<a href=\"{}\">{}</a>", link, content);
        self.links.push(tag_ancora);
    }

    pub fn new() -> Self {
        let mut string_style = fs::read_to_string(format!(
            "{}{}",
            env::var("PATH_STYLES_SHEETS").expect("Não foi possivel ler variavel de ambiente"),
            "style_index.css"
        ))
        .expect("Não foi possivel ler arquivo css");

        string_style = format!("<style>{}</style>", string_style);

        TemplateIndex {
            style_sheet: string_style,
            links: Vec::new(),
            link_request_about: format!("{}about.md", env::var("LINK_REQUEST_RELEASE").unwrap()),
        }
    }
}
