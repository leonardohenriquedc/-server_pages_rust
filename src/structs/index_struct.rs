use std::fs;

use askama::Template;

const PATH_STYLES_SHEETS: &str = "templates/styles/";
#[derive(Template)]
#[template(path = "layout/index.html")]
pub struct TemplateIndex {
    pub style_sheet: String,
    links: Vec<String>,
}

impl TemplateIndex {
    pub fn insert_tag_ancora(&mut self, link: String, content: String) {
        let tag_ancora = format!("\n<a href=\"{}\">{}</a>", link, content);
        self.links.push(tag_ancora);
    }

    pub fn new() -> Self {
        let mut string_style =
            fs::read_to_string(format!("{}{}", PATH_STYLES_SHEETS, "style_index.css"))
                .expect("Não foi possivel ler arquivo");

        string_style = format!("<style>{}</style>", string_style);

        TemplateIndex {
            style_sheet: string_style,
            links: Vec::new(),
        }
    }
}
