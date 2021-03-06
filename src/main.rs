// For listing .md files of blog.
extern crate walkdir;
use walkdir::WalkDir;

use pulldown_cmark::{html, Parser};
use std::collections::HashMap;

// Set this macro rule for defining value of hash map in definition.
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

// This function will return list of blog files.
fn get_files_list() -> Vec<String> {
    let mut blogs_list: Vec<String> = Vec::new();
    // Iterating on blog files recursively.
    for entry in WalkDir::new("./blog").into_iter().filter_map(|e| e.ok()) {
        if entry.metadata().unwrap().is_file() {
            blogs_list.push(entry.path().display().to_string());
        }
    }
    blogs_list
}

fn create_result_directory() {
    if std::path::Path::new(&"./result").is_dir() == true {
        std::fs::remove_dir_all("./result").expect("Cannot remove result directory");
    } else {
        std::fs::create_dir("./result").expect("Cannot create result directory");
        let options = fs_extra::dir::CopyOptions::new();
        fs_extra::dir::copy("./templates/", "./result/", &options)
            .expect("Cannot copy templates to result directory");
    }
}

// Wrap blog data into html template.
fn wrap_html(html_data: HashMap<String, String>) -> String {
    let html_page = format!(
        r#"<!DOCTYPE html>

<html>
  <head>
    <meta name="viewport" content="width=device-width, initial-scale=1.0">

    <link rel="stylesheet" href="{main_css}">
    <link rel="stylesheet" href="{code_css}">
    <link rel="stylesheet" href="{prism_css}">

    <title>{title}</title>
  </head>

  <body>
    <div class="main">
      {blog_content}
    </div>

    <script src="../templates/libraries/prism/prism.js"></script>
  </body>
</html>"#,
        title = html_data.get("title").unwrap(),
        main_css = html_data.get("main_css").unwrap(),
        code_css = html_data.get("code_css").unwrap(),
        prism_css = html_data.get("prism_css").unwrap(),
        blog_content = html_data.get("blog_content").unwrap(),
    )
    .to_string();

    html_page
}

fn main() {
    get_files_list();
    create_result_directory();

    for file in get_files_list() {
        // Reading markdown file, converting it in html and cloning it's content
        // in blog_content variable.
        let markdown_file_content = std::fs::read_to_string(&file)
            .expect("Cannot open one or more of your blog file/files");
        let parser = Parser::new(&markdown_file_content);
        let mut result_string = String::new();
        html::push_html(&mut result_string, parser);
        let title = result_string.lines().nth(0).unwrap();
        let title = &title[2..];

        // HashMap that it's values used in html template string.
        let html_data = hashmap![
            "title".to_string() => title.to_string(),
            "main_css".to_string() => "./../templates/css/main.css".to_string(),
            "code_css".to_string() => "./../templates/css/code.css".to_string(),
            "prism_css".to_string() => "./../templates/libraries/prism/prism.css".to_string()
        ];

        // Finding future html file name.
        let html_file_name = std::path::Path::new(&file)
            .file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap();
        let html_file_name: Vec<&str> = html_file_name.split(".").collect();
        let mut output = String::new();
        for part in 0..html_file_name.len() - 1 {
            output.push_str(html_file_name[part]);
        }
        output.push_str(".html");
        let html_file_name = output.clone();
        drop(&output);

        // Finding future html file parent directory name.
        let html_file_parent_directory: Vec<&str> = file.split("/").collect();
        let directory_name = html_file_parent_directory[html_file_parent_directory.len() - 2];
        drop(&html_file_parent_directory);

        // Creating html file parent directory.
        if std::path::Path::new(&"./result/directory_output").is_dir() == false {
            std::fs::create_dir(format!("./result/{}", directory_name))
                .expect("Cannot create html file parent directory");
        }

        // Adding blog content to html template and writing to it's file.
        let final_html = wrap_html(html_data);
        std::fs::write(
            format!("result/{}/{}", directory_name, html_file_name),
            final_html,
        )
        .expect("Cannot write to result.html file");
    }
}
