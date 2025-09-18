use std::{
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Read, Write},
};

use actix_web::ResponseError;
use pulldown_cmark::{Options, Parser, html};

const PATH_FILES_BLOGS: &str = "src/files/blogs/";
const NAME_INDEX_PAGE: &str = "index.md";

pub async fn hello_word() -> String {
    write_block().await;

    let mut file = File::open(format!("{}{}", PATH_FILES_BLOGS, NAME_INDEX_PAGE))
        .expect("Arquivo não encontrado");

    let mut content = String::new();

    let _ = file
        .read_to_string(&mut content)
        .expect("Não foi possivel ler conteudo");

    let mut options = Options::empty();

    options.insert(Options::ENABLE_STRIKETHROUGH);

    let content_as_str = content.as_str();

    let parser = Parser::new_ext(content_as_str, options);

    let mut content_as_html = String::new();

    html::push_html(&mut content_as_html, parser);

    content_as_html
}

async fn write_block() {
    let lines_modified = insert_link_of_blogs().await;

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

async fn get_block_of_links() -> Vec<String> {
    let mut block_of_link: Vec<String> = Vec::new();

    let file = File::open(format!("{}{}", PATH_FILES_BLOGS, NAME_INDEX_PAGE))
        .expect("Não foi possivel ler arquivo");

    let buf_reader = BufReader::new(file);

    for line_result in buf_reader.lines() {
        let line = line_result.expect("Não foi possivel ler linha");
        block_of_link.push(line);
    }

    println!("Esse e o block_of_link: {:?}", block_of_link);

    block_of_link
}

async fn insert_link_of_blogs() -> Vec<String> {
    let name_of_blogs = get_names_blogs().await;

    let mut vec_file = get_block_of_links().await;

    let indice_div = vec_file
        .clone()
        .iter()
        .position(|s| s.trim().starts_with("<div id=\"container\""))
        .expect("Valor não encontrado");

    let indice_end_div = vec_file
        .clone()
        .iter()
        .position(|s| s.trim().starts_with("</div"))
        .expect("Valor não encontrado");

    println!(
        "Esse foi o indique encontrado dentro do indice_div: {} e quando resgato e esse o valor: {:?}",
        indice_div,
        vec_file.get(indice_div)
    );

    for (i, line) in vec_file.clone()[indice_div..].iter().enumerate() {
        let mut modify = false;

        println!("{i}:{line}");

        for name in name_of_blogs.clone() {
            let tag_anconra = format!(
                "\n <a href=\"http://localhost:7878/{}\">{}</a> \n",
                name, name
            );

            if line.starts_with("<a") && line.clone() == tag_anconra.clone() {
                modify = false;
                continue;
            }

            if line.starts_with("</div") && modify == false {
                println!(
                    "Entrou na segunda condicional e foi true: {} e esse e o indique: {}",
                    line, i
                );

                println!(
                    "Quando pego o i e puxo o item dessa posição: {:?}",
                    vec_file.get(i)
                );
                vec_file.insert(indice_end_div, tag_anconra.clone());
                modify = true;
                continue;
            }
        }
    }

    //println!("Esse e o vec_file: {:?}", vec_file);

    vec_file
}

async fn get_names_blogs() -> Vec<String> {
    let mut name_blogs: Vec<String> = Vec::new();

    let read_dir = fs::read_dir(PATH_FILES_BLOGS).expect("Não foi possivel localizar pasta");

    for entry in read_dir {
        if entry
            .as_ref()
            .ok()
            .unwrap()
            .file_name()
            .into_string()
            .unwrap()
            == NAME_INDEX_PAGE
        {
            continue;
        }

        match entry {
            Ok(valor) => name_blogs.push(valor.file_name().into_string().unwrap()),
            Err(error) => print!("Error occurred: {}", error.status_code()),
        }
    }

    name_blogs
}
