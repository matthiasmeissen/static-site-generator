
use std::{fs, path::Path};
use pulldown_cmark;
use tera::{Context, Tera};
use lol_html::{element, HtmlRewriter, Settings, html_content::{ContentType, Element}}; 


const CONTENT_DIR: &str = "src/content";
const STATIC_DIR: &str = "src/static";
const OUTPUT_DIR: &str = "dist";
const TEMPLATE_DIR: &str = "src/templates/**/*.html";


fn main() {
    let tera = Tera::new(TEMPLATE_DIR).unwrap();

    prepare_output_dir();

    process_file("index.html", "index.html", &tera);
    process_file("bike-tribals.md", "bike-tribals.html", &tera);

    copy_static_file("global.css");
    copy_static_file("components.css");
    copy_static_file("bike-tribal.svg");
}


// Markdown to html

fn md_to_html(md: &str) -> String {
    let parser = pulldown_cmark::Parser::new(md);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    html_output
}

fn inject_html_components(html_content: &str, tera: &Tera) -> String {
    let mut output = vec![];

    let mut rewriter = HtmlRewriter::new(
        Settings {
            element_content_handlers: vec![
                element!("info-card", |el| {
                    render_component(el, tera, "components/info_card.html")
                }),

                element!("project-teaser", |el| {
                    render_component(el, tera, "components/project-teaser.html")
                }),
            ],
            ..Settings::default()
        },
        |c: &[u8]| output.extend_from_slice(c),
    );

    rewriter.write(html_content.as_bytes()).unwrap();
    rewriter.end().unwrap();

    String::from_utf8(output).unwrap()
}

fn render_component(el: &mut Element, tera: &Tera, template_name: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut context = Context::new();

    for attr in el.attributes() {
        context.insert(attr.name(), &attr.value());
    }

    let rendered = tera.render(template_name, &context)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

    el.replace(&rendered, ContentType::Html);

    Ok(())
}

fn inject_into_template(input: &str, html_template: &str, tera: &Tera) -> String {
    let mut context = Context::new();
    context.insert("content", &input);

    tera.render(html_template, &context).unwrap()
}

fn process_file(filename: &str, output_name: &str, tera: &Tera) {
    let input_path = format!("{}/{}", CONTENT_DIR,filename);
    let file_content = fs::read_to_string(&input_path).expect(&format!("Failed to read {}", input_path));

    let final_content = if filename.ends_with(".md") {
        let raw_html = md_to_html(&file_content);
        inject_html_components(&raw_html, tera) 
    } else {
        let raw_html = file_content;
        inject_html_components(&raw_html, tera) 
    };

    let final_page = inject_into_template(&final_content, "base.html", tera);

    fs::write(format!("{}/{}", OUTPUT_DIR, output_name), final_page).expect("Could not write output file.")
}


// Utilities

fn prepare_output_dir() {
    if Path::new(OUTPUT_DIR).exists() {
        fs::remove_dir_all(OUTPUT_DIR).expect("Failed to clear output directory.");
    }
    fs::create_dir_all(OUTPUT_DIR).expect("Failed to create dist directory.");
}

fn copy_static_file(file_name: &str) {
    let source = format!("{STATIC_DIR}/{file_name}");
    let dest = format!("{OUTPUT_DIR}/{file_name}");
    fs::copy(source, dest).expect("Could not copy static file.");
}
