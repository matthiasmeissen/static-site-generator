# Static Site Generator

A simple static site generator written in rust.

## Intro

This website is built with a custom ssg written in rust using: 
- `pulldown_cmark` for md parsing
- `tera` for html templating
- `lol_html` for html replacement

## Usage

### Content and Templates

Create html or md files in the `src/content` folder.

Add html templates in `src/templates` folder.
Inside an html template add `{{ content | safe }}` once to specify where you want the content to be placed.

### Static files

Everything you put into the `src/static` folder will be copied as is to `dist`.
This is usefull for css files or assets like fonts or images.

### Html Components

You can create custom components and insert them into md or html content files.

Create the component in `src/templates/components` by writing the html you want.
Add attributess by writing {{ attribut-name | default(value="Some value") }} wherever you want in the file.
As you can see, an attribute can have a default value with the syntax shown above.

To place it into a content file, a custom tag of your choice.
Make sure it uses opening and closing tags like `<my-component title="My title" text="Some text in here"></my-component>`

The last step is to add the component to the `main.rs` file.
In the inject_html_components() function, there is a rewriter variable.
Add your element in the same style to the collection.

```
element!("info-card", |el| {
    render_component(el, tera, "components/info_card.html")
}),
```

### Build the page

In the main function in `main.rs` call `process_file(<content_file_name>, "<output_file_name>", &tera);` for each file you want to create.
A md file it will be converted to html and inserted inti the template.
A html file be taken as is and put into the template.

Then run `cargo run` to build the files, which are stored in the `dist` folder.