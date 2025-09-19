use std::{
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Read, Write},
    vec,
};

use actix_web::ResponseError;
use pulldown_cmark::{Options, Parser, html};

const PATH_FILES_BLOGS: &str = "src/files/blogs/";
const NAME_INDEX_PAGE: &str = "index.md";

pub async fn get_links_of_all_blogs() -> String {
    write_block(insert_lines_of_file_with_parameterization().await).await;

    let mut file = File::open(format!("{}{}", PATH_FILES_BLOGS, NAME_INDEX_PAGE))
        .expect("Arquivo não encontrado");

    let mut content = String::new();

    let _ = file
        .read_to_string(&mut content)
        .expect("Não foi possivel ler conteudo");

    content = parsing_md_to_html(content.as_str());

    content
}

pub async fn get_blog_post(name: String) -> String {
    let path = format!("{}{}", PATH_FILES_BLOGS, name);

    let path_page_blog = format!("{}{}", PATH_FILES_BLOGS, "page_blog.md");

    let file = File::open(path.clone()).expect("Arquivo não encontrado");

    let buf_reader = BufReader::new(file);

    let mut vec_lines: Vec<String> = buf_reader
        .lines()
        .map(|s| s.expect("Não foi possivel ler linha"))
        .collect();

    let mut content = vec_lines.join("\n");

    content = parsing_md_to_html(content.as_str());

    vec_lines = content.split("\n").map(|s| s.to_string()).collect();

    let mut vec_file = get_block_of_file(path_page_blog.as_str()).await;

    vec_file =
        insert_lines_of_file_no_parameterization("</div>".to_string(), vec_file, vec_lines).await;

    vec_file.join("\n")
}

fn parsing_md_to_html(content: &str) -> String {
    let mut options = Options::empty();

    options.insert(Options::ENABLE_STRIKETHROUGH);

    let parser = Parser::new_ext(content, options);

    let mut content_as_html = String::new();

    html::push_html(&mut content_as_html, parser);

    content_as_html
}

async fn write_block(vector: Vec<String>) {
    let lines_modified = vector;

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(format!("{}{}", PATH_FILES_BLOGS, NAME_INDEX_PAGE))
        .expect("Não foi possivel abrir arquivo");

    for line in lines_modified {
        writeln!(file, "{}", line).expect("Não foi possivel escrever");
    }
}

async fn get_block_of_file(path: &str) -> Vec<String> {
    let mut block_of_link: Vec<String> = Vec::new();

    let file = File::open(path).expect("Não foi possivel ler arquivo");

    let buf_reader = BufReader::new(file);

    for line_result in buf_reader.lines() {
        let line = line_result.expect("Não foi possivel ler linha");
        block_of_link.push(line);
    }

    block_of_link
}

async fn insert_lines_of_file_no_parameterization(
    name_end_tag: String,
    mut vec_file: Vec<String>,
    vec_lines: Vec<String>,
) -> Vec<String> {
    let indice_end_div = vec_file
        .clone()
        .iter()
        .position(|s| s.trim().starts_with(name_end_tag.as_str()))
        .expect("Valor não encontrado");

    for line in (0..vec_lines.len()).rev() {
        vec_file.insert(indice_end_div, vec_lines[line].clone());
    }

    vec_file
}

async fn insert_lines_of_file_with_parameterization() -> Vec<String> {
    let name_of_blogs = get_name_files().await;

    let mut vec_file =
        get_block_of_file(format!("{}{}", PATH_FILES_BLOGS, NAME_INDEX_PAGE).as_str()).await;

    let indice_end_div = vec_file
        .clone()
        .iter()
        .position(|s| s.trim().starts_with("</div"))
        .expect("Valor não encontrado");

    for name in name_of_blogs {
        let tag_ancora = format!(
            "<a href=\"http://localhost:7878/post/{}\">{}</a>",
            name, name
        );

        // Verifica se já existe no arquivo
        let already_exists = vec_file.iter().any(|line| line.trim() == tag_ancora);

        if !already_exists {
            println!("Inserindo novo link: {}", tag_ancora);
            vec_file.insert(indice_end_div, tag_ancora);
        } else {
            println!("Link já existe: {}", tag_ancora);
        }
    }
    vec_file
}

async fn get_name_files() -> Vec<String> {
    let mut name_blogs: Vec<String> = Vec::new();

    let read_dir = fs::read_dir(PATH_FILES_BLOGS).expect("Não foi possivel localizar pasta");

    for entry in read_dir {
        let entri = entry
            .as_ref()
            .ok()
            .unwrap()
            .file_name()
            .into_string()
            .unwrap();
        if entri == NAME_INDEX_PAGE || entri == "page_blog.md" {
            continue;
        }

        match entry {
            Ok(valor) => name_blogs.push(valor.file_name().into_string().unwrap()),
            Err(error) => print!("Error occurred: {}", error.status_code()),
        }
    }

    name_blogs
}
