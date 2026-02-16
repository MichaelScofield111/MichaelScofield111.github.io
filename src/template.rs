use handlebars::{DirectorySourceOptionsBuilder, Handlebars};
use serde_json::json;

use crate::post::Post;

pub struct TemplateEngine {
    pub hb: Handlebars<'static>,
}

impl TemplateEngine {
    pub fn new() -> Self {
        let mut handlebars = Handlebars::new();
        handlebars
            .register_templates_directory(
                "./templates",
                DirectorySourceOptionsBuilder::default()
                    .tpl_extension(".hbs")
                    .build()
                    .expect("fail to build directory source options"),
            )
            .expect("fail to register templates file");

        Self { hb: handlebars }
    }

    pub fn render_post(&self, post: &Post) -> String {
        self.hb
            .render(
                "post",
                &json!({
                    "title": post.meta.title,
                    "description": post.meta.description,
                    "author": post.meta.author,
                    "cover": post.meta.cover,
                    "content": post.html
                }),
            )
            .unwrap()
    }

    pub fn render_index(&self, posts: &[Post]) -> String {
        let post_data: Vec<_> = posts
            .iter()
            .map(|p| {
                serde_json::json!({
                    "title": p.meta.title,
                    "description": p.meta.description,
                    "author": p.meta.author,
                    "slug": p.slug,
                })
            })
            .collect();

        self.hb
            .render(
                "index",
                &serde_json::json!({
                    "posts": post_data
                }),
            )
            .unwrap()
    }
}
