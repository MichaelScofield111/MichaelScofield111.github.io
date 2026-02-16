use crate::frontmatter::FrontMatter;

#[derive(Debug)]
pub struct Post {
    pub meta: FrontMatter,
    pub html: String,
    pub slug: String,
}

impl Post {
    pub fn from_markdown(content: &str, slug: String) -> Self {
        // a safe method to parse front_matter and markdown_content
        let (front_matter, markdown_content) =
            markdown_frontmatter::parse::<FrontMatter>(content).unwrap();
        let parser = pulldown_cmark::Parser::new(markdown_content);
        // Write to a new String buffer.
        let mut html_output = String::new();
        pulldown_cmark::html::push_html(&mut html_output, parser);
        Self {
            meta: front_matter,
            html: html_output,
            slug,
        }
    }
}
