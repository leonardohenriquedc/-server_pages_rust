use askama::Template;

#[derive(Template)]
#[template(path = "layout/page_view_article.html")]
pub struct TemplatePageViewArticle {
    pub style_sheet: String,
    pub title: String,
    pub content: String,
}
impl TemplatePageViewArticle {
    pub fn new() -> Self {
        TemplatePageViewArticle {
            style_sheet: String::new(),
            title: String::new(),
            content: String::new(),
        }
    }
}
