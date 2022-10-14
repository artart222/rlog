use maud::{html, PreEscaped, DOCTYPE};
use pulldown_cmark::{html, Parser};

fn wrap_html(
    blog_content: String,
    main_css: String,
    code_css: String,
    prism_css: String,
    title: String,
) -> String {
    let webpage = html! {
        (DOCTYPE)  // <!DOCTYPE html>
        html {
          head {
            meta name="viewport" content="width=device-width, initial-scale=1.0" {}
            link rel="stylesheet" href=(main_css) {}
            link rel="stylesheet" href=(code_css) {}
            link rel="stylesheet" href=(prism_css) {}
            title {(title)} // Put title for website.
          }
          body {
            div."header" {
              div."logo-header" {
                div."logo-container" {
                  a href="home.html" {
                    img src="images/Profile-icons/artart222300x.png"
                    alt="home"
                    style="max-width: 1.57em; max-height: 1.57em; width: 100%; height: 100%;" {}
                  }
                }
              }
              div."navigation-header" {
                div."links-container" {
                  a href="home.html" {"home"}
                  a href="tutorials.html" {"tutorials"}
                  a href="blog.html" {"blog"}
                  a href="https://github.com/artart222"
                    target="_blank" {
                    img
                      src="images/GitHub/GitHub-Mark-Light-120px-plus.png"
                      alt="GitHub"
                      style="max-width: 1.57em; max-height: 1.57em; width: 100%; height: 100%;" {}
                  }
                  a href="https://discord.gg/GhwVQn5YFU"
                    target="_blank" {
                      img
                        src="images/Discord/Discord512px.png"
                        alt="Discord"
                        style="max-width: 1.57em; max-height: 1.57em; width: 100%; height: 100%;" {}
                  }
                  a
                    href="https://www.youtube.com/channel/UCQSDE-p625gJdNmKLYGSqKA/featured"
                    target="_blank" {
                      img
                        src="images/Youtube/youtube64x.png"
                        alt="YouTube"
                        style="max-width: 1.57em; max-height: 1.57em; width: 100%; height: 100%;" {}
                  }
                }
              }
            }
            div."main" {
              (PreEscaped(blog_content))
            }
            script src="./templates/libraries/prism/prism.js" {}
          }
        }
    }
    .into_string();
    return webpage;
}

// Argument is config of home page.
// This will be in config.toml file.
// This function will read home section of config file
// and it will convert them to html strings.
pub fn make_homepage(
    homepage_config: Vec<Vec<String>>,
    main_css: String,
    code_css: String,
    prism_css: String,
    title: String,
) -> String {
    println!("Making \"index.html\"");
    let mut content = "<h1>".to_string() + &title + &"</h1>".to_string();
    for item in homepage_config {
        content = content + &"<div class=\"content-desc\">".to_string();
        if item.len() == 2 {
            content = content + &"<img src=\"".to_string() + &item[1] + &"\" />".to_string();
        }
        let parser = Parser::new(&item[0]);
        let mut result_string = String::new();
        html::push_html(&mut result_string, parser);
        content = content + &result_string + "</div>";
    }
    return wrap_html(content, main_css, code_css, prism_css, title);
}

pub fn make_blogpage(
    blog_content: String,
    main_css: String,
    code_css: String,
    prism_css: String,
    title: String,
) -> String {
    println!("Making \"{}\"", title);
    // let mut blog_content_copy = blog_content.clone();
    // println!("{}", blog_content_copy);
    // let parser = Parser::new(&blog_content);
    // html::push_html(&mut blog_content_copy, parser);
    // return wrap_html(blog_content_copy, main_css, code_css, prism_css, title);

    let mut blog_content_copy = String::new();
    println!("{}", blog_content_copy);
    let parser = Parser::new(&blog_content);
    html::push_html(&mut blog_content_copy, parser);
    return wrap_html(blog_content_copy, main_css, code_css, prism_css, title);
}
