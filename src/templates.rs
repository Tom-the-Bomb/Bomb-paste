use askama::Template;


#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {}


#[derive(Template)]
#[template(path = "help.html")]
pub struct Help {
    pub min_content_length: usize,
    pub max_content_length: usize,
}


#[derive(Template)]
#[template(path = "notfound.html")]
pub struct NotFound {}


#[derive(Template)]
#[template(path = "view_paste.html")]
pub struct Paste<'a> {
    pub paste_content: &'a str,
}