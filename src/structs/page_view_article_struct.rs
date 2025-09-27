use std::{env, fs};

use askama::Template;

#[derive(Template)]
#[template(path = "layout/page_view_article.html")]
pub struct TemplatePageViewArticle {
    link_home: String,
    pub style_sheet: String,
    pub title: String,
    pub content: String,
}
impl TemplatePageViewArticle {
    pub fn new() -> Self {
        let mut style = fs::read_to_string(format!(
            "{}{}",
            env::var("PATH_STYLES_SHEETS").expect("Não foi possivel ler variavel de ambiente"),
            "style_page_view.css"
        ))
        .expect("Não foi possivel ler arquivo css");

        style = format!("<style>{}</style>", style);

        TemplatePageViewArticle {
            link_home: env::var("LINK_REQUEST_RELEASE").expect("Não foi possivel ler variavel"),
            style_sheet: style,
            title: String::new(),
            content: String::new(),
        }
    }
}
