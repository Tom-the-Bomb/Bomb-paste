use askama::Template;


#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {}


#[derive(Template)]
#[template(path = "help.html")]
pub struct Help {}


#[derive(Template)]
#[template(path = "notfound.html")]
pub struct NotFound {}


#[derive(Template)]
#[template(path = "view_paste.html")]
pub struct Paste<'a> {
    pub paste_content: &'a str,
}