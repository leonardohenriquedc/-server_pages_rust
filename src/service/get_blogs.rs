use std::env;
use std::fs::{self};

use crate::structs::{
    index_struct::TemplateIndex, page_view_article_struct::TemplatePageViewArticle,
};

use actix_web::ResponseError;
use askama::Template;
use pulldown_cmark::{Options, Parser, html};

fn return_envs_links(type_number: i8) -> String {
    let path_files_blogs =
        env::var("PATH_FILES_BLOGS").expect("não foi possivel ler variavel de ambiente");
    let link =
        env::var("LINK_REQUEST_REQUEST").expect("Não foi possivel ler variaveis de ambiente");

    match type_number {
        1 => path_files_blogs,
        _ => link,
    }
}

pub async fn get_links_of_all_blogs() -> String {
    let mut template_index = TemplateIndex::new();

    let name_blogs = get_name_files().await;

    for name in name_blogs {
        let link = format!("{}post/{}", return_envs_links(2), name);
        let name_format = name.strip_suffix(".md").unwrap().to_string();
        template_index.insert_tag_ancora(link, name_format);
    }

    template_index.render().unwrap()
}

pub async fn get_blog_post(name: String) -> String {
    let path_article = format!("{}{}", return_envs_links(1), name);

    let article_content =
        fs::read_to_string(path_article.as_str()).expect("Não foi possivel ler arquivo");

    let content = parsing_md_to_html(&article_content);

    let mut template_view = TemplatePageViewArticle::new();

    template_view.title = name
        .strip_suffix(".md")
        .expect("Erroe ao remover sufixo")
        .to_string();
    template_view.content = content;

    template_view
        .render()
        .expect("Não foi possivel renderizar html")
}

fn parsing_md_to_html(content: &str) -> String {
    let mut options = Options::empty();

    options.insert(Options::ENABLE_STRIKETHROUGH);

    let parser = Parser::new_ext(content, options);

    let mut content_as_html = String::new();

    html::push_html(&mut content_as_html, parser);

    content_as_html
}

async fn get_name_files() -> Vec<String> {
    let mut name_blogs: Vec<String> = Vec::new();

    let read_dir = fs::read_dir(return_envs_links(1)).expect("Não foi possivel localizar pasta");

    for entry in read_dir {
        match entry {
            Ok(valor) => name_blogs.push(valor.file_name().into_string().unwrap()),
            Err(error) => print!("Error occurred: {}", error.status_code()),
        }
    }

    name_blogs
}
