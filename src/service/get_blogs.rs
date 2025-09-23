use std::collections::HashMap;
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
        env::var("LINK_REQUEST_RELEASE").expect("Não foi possivel ler variaveis de ambiente");
    match type_number {
        1 => path_files_blogs,
        _ => link,
    }
}

pub async fn get_links_of_all_blogs() -> String {
    let mut template_index = TemplateIndex::new();

    let name_blogs = get_name_files().await;

    for (name, title) in name_blogs {
        template_index.insert_tag_ancora(format!("{}post/{}", return_envs_links(2), name), title);
    }

    template_index.render().unwrap()
}

pub async fn get_blog_post(name: String) -> String {
    let path_article = format!("{}{}", return_envs_links(1), name);

    let mut article_content =
        fs::read_to_string(path_article.as_str()).expect("Não foi possivel ler arquivo");

    let mut title = String::new();

    for (index, line) in article_content.clone().lines().enumerate() {
        if line == "---" && index != 0 {
            article_content = article_content.replace(line, "");
            break;
        }

        if line.starts_with("title: ") {
            title = line
                .strip_prefix("title: ")
                .expect("Não foi possivel remover prefixo")
                .to_string();
        }

        article_content = article_content.replace(line, "");
    }

    let content = parsing_md_to_html(&article_content);

    let mut template_view = TemplatePageViewArticle::new();

    template_view.title = title;
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

async fn get_name_files() -> HashMap<String, String> {
    let mut name_title_file_blogs: HashMap<String, String> = HashMap::new();

    let read_dir = fs::read_dir(return_envs_links(1)).expect("Não foi possivel localizar pasta");

    for entry in read_dir {
        match entry {
            Ok(valor) => {
                name_title_file_blogs.insert(
                    valor.file_name().into_string().unwrap(),
                    get_title_of_file(
                        fs::read_to_string(valor.path()).expect("Não foi possivel ler arquivo"),
                        "title: ".to_string(),
                    ),
                );
            }
            Err(error) => print!("Error occurred: {}", error.status_code()),
        }
    }

    name_title_file_blogs
}

fn get_title_of_file(file: String, metadado: String) -> String {
    let mut metadados_dois: Vec<String> = Vec::new();

    for (index, line) in file.lines().enumerate() {
        if line == "---" && index != 0 {
            break;
        }

        if line != "---" {
            metadados_dois.push(line.to_string());
        }
    }

    if !metadado.is_empty() {
        for line in &metadados_dois {
            if line.starts_with(&metadado) {
                return line
                    .clone()
                    .strip_prefix(&metadado)
                    .expect("não foi possivel remover prefixo")
                    .to_string();
            }
        }
    }

    metadados_dois.join("\n")
}
