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
    println!();
    println!();
    let mut blogs_list: Vec<String> = Vec::new();
    // Iterating on blog files recursively.
    for entry in WalkDir::new("./blog")
        .into_iter()
        .filter_map(|file| file.ok())
    {
        // If item is file.
        if entry.metadata().unwrap().is_file() {
            // If it has extension.
            if entry.path().extension().is_some() {
                // If the extension is markdown.
                if entry.path().extension().unwrap() == "md" {
                    println!(
                        "Adding {} to list of blog files that will convert to html.",
                        entry.path().display().to_string()
                    );
                    // push it to blogs_list.
                    blogs_list.push(entry.path().display().to_string());
                };
            }
        }
    }
    blogs_list
}

fn create_result_directory() {
    // Remove and remake result directory if it already exist.
    if std::path::Path::new(&"./result").is_dir() == true {
        println!("Removing existing result directory.");
        std::fs::remove_dir_all("./result").expect("Cannot remove result directory");
        println!("Creating new result directory.");
        std::fs::create_dir("./result").expect("Cannot create result directory");
    } else {
        // Make result directory if it doesn't exist.
        println!("Creating new result directory.");
        std::fs::create_dir("./result").expect("Cannot create result directory");
    }
    // Move templates directory to result.
    let options = fs_extra::dir::CopyOptions::new();
    fs_extra::dir::copy("./templates", "./result", &options)
        .expect("Cannot copy templates to result directory");

    // Iterating over items in ./blog unrecursively.
    for path in std::fs::read_dir("./blog").unwrap() {
        let path = path.unwrap();
        println!(
            "Moving {} to result directory.",
            path.path().display().to_string()
        );
        // Moving items to result directory based on being a file or directory.
        if path.metadata().unwrap().is_dir() {
            let options = fs_extra::dir::CopyOptions::new();
            fs_extra::dir::copy(path.path().display().to_string(), "./result", &options)
                .expect(&format!("Cannot copy {}", path.path().display().to_string()).to_string());
        } else {
            let options = fs_extra::file::CopyOptions::new();
            fs_extra::file::copy(
                path.path().display().to_string(),
                format!("./result/{}", path.file_name().to_str().unwrap()),
                &options,
            )
            .expect(&format!("Cannot copy {}", path.path().display().to_string()).to_string());
        }
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
    <div class="header">
      <div class="logo-header">
        <div class="logo-container">
          <a href="home.html"
            ><img
              src="images/Profile-icons/artart222300x.png"
              alt="home"
              style="
                max-width: 1.57em;
                max-height: 1.57em;
                width: 100%;
                height: 100%;
              "
          /></a>
        </div>
      </div>

      <div class="navigation-header">
        <div class="links-container">
          <a href="home.html">home</a>
          <a href="tutorials/tutorials.html">tutorials</a>
          <a href="blog.html">blog</a>

          <a href="https://github.com/artart222" target="_blank"
            ><img
              src="images/GitHub/GitHub-Mark-Light-120px-plus.png"
              alt="GitHub"
              style="
                max-width: 1.57em;
                max-height: 1.57em;
                width: 100%;
                height: 100%;
              "
          /></a>
          <a href="https://discord.gg/GhwVQn5YFU" target="_blank"
            ><img
              src="images/Discord/Discord512px.png"
              alt="Discord"
              style="
                max-width: 1.57em;
                max-height: 1.57em;
                width: 100%;
                height: 100%;
              "
          /></a>
          <a
            href="https://www.youtube.com/channel/UCQSDE-p625gJdNmKLYGSqKA/featured"
            target="_blank"
            ><img
              src="images/Youtube/youtube64x.png"
              alt="YouTube"
              style="
                max-width: 1.57em;
                max-height: 1.57em;
                width: 100%;
                height: 100%;
              "
          /></a>
        </div>
      </div>
    </div>

    <div class="main">
      {blog_content}
    </div>

    <script src="templates/libraries/prism/prism.js"></script>
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
    create_result_directory();

    for file in get_files_list() {
        // Reading markdown file, converting it in html.
        let markdown_file_content = std::fs::read_to_string(&file)
            .expect("Cannot open one or more of your blog file/files");
        let parser = Parser::new(&markdown_file_content);
        let mut result_string = String::new();
        html::push_html(&mut result_string, parser);

        // Assuming first line of all .md files is blog title.
        let title = result_string.lines().nth(0).unwrap(); // First line of '&file'.
        let title = &title[2..]; // Removing '# ' from beginning of title string.

        // HashMap that it's values used in html template string.
        let html_data = hashmap![
            "title".to_string() => title.to_string(),
            "main_css".to_string() => "./../templates/css/main.css".to_string(),
            "code_css".to_string() => "./../templates/css/code.css".to_string(),
            "prism_css".to_string() => "./../templates/libraries/prism/prism.css".to_string(),
            "blog_content".to_string() => result_string.to_string()
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

        // Adding blog content to html template and writing to it's file.
        let final_html = wrap_html(html_data);
        std::fs::write(format!("result/{}", html_file_name), final_html)
            .expect("Cannot write to result.html file");
    }
}
