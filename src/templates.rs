use askama::Template;


#[derive(Template)]
#[template(path = "../static/templates/index.html")]
pub struct Index {}


#[derive(Template)]
#[template(path = "../static/templates/notfound.html")]
pub struct NotFound {}


#[derive(Template)]
#[template(path = "../static/templates/view_paste.html")]
pub struct Paste<'a> {
    pub paste_content: &'a str,
}