use askama::Template;


#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {}


#[derive(Template)]
#[template(path = "help.html")]
pub struct Help {
    pub min_content_length: usize,
    pub max_content_length: usize,
    pub max_upload_rate: u64,
    pub max_upload_per: u64,
}


#[derive(Template)]
#[template(path = "notfound.html")]
pub struct NotFound {}

#[derive(Template)]
#[template(path = "internalerror.html")]
pub struct InternalError {}


#[derive(Template)]
#[template(path = "view_paste.html")]
pub struct Paste<'a> {
    pub paste_content: &'a str,
}