use std::env;

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
        TemplatePageViewArticle {
            link_home: env::var("LINK_REQUEST_RELEASE").expect("Não foi possivel ler variavel"),
            style_sheet: String::new(),
            title: String::new(),
            content: String::new(),
        }
    }
}
