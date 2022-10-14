// For listing .md files of blog.
extern crate walkdir;
use walkdir::WalkDir;

// For reading config file.
mod config;
use crate::config::read_config;
// For converting strings inputs to html strings.
mod html_generator;
use crate::html_generator::{make_blogpage, make_homepage};

// This function will return list of blog files.
fn get_files_list() -> Vec<String> {
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
    println!();
    println!();
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

fn main() {
    create_result_directory();

    println!();
    println!();

    for file in get_files_list() {
        // Reading markdown file, converting it in html.
        let markdown_file_content = std::fs::read_to_string(&file)
            .expect("Cannot open one or more of your blog file/files");

        // Assuming first line of all .md files is blog title.
        let title = markdown_file_content.lines().nth(0).unwrap(); // First line of '&file'.
        let title = &title[2..].to_string(); // Removing '# ' from beginning of title string.

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
        // Wrap blog data into html template.
        let main_css = "./templates/css/main.css".to_string();
        let code_css = "./templates/css/code.css".to_string();
        let prism_css = "./templates/libraries/prism/prism.css".to_string();
        let blog_page = make_blogpage(
            markdown_file_content.clone(),
            main_css,
            code_css,
            prism_css,
            title.to_string(),
        );

        std::fs::write(format!("result/{}", html_file_name), blog_page)
            .expect("Cannot write to result.html file");
    }

    let homepage = make_homepage(
        read_config().home,
        String::from("./templates/css/main.css"),
        String::from("./templates/css/code.css"),
        String::from("./templates/libraries/prism/prism.css"),
        String::from("Home"),
    );
    std::fs::write("result/home.html", homepage).expect("Cannot write to result.html file");
}
