use crate::{post::Post, template::TemplateEngine};
use anyhow::{Ok, Result};
use std::{fs, path::Path};
use walkdir::WalkDir;

pub fn build() -> Result<()> {
    // to store post
    let mut posts = vec![];

    // clean old post and build new post
    clean_old_posts()?;

    // copy static files to public
    move_static_files()?;

    for entry in WalkDir::new("content") {
        let entry = entry.unwrap();
        if entry.path().extension().and_then(|s| s.to_str()) == Some("md") {
            // is the markdown file
            let content = fs::read_to_string(entry.path()).unwrap();
            let slug = entry
                .path()
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .to_string();

            let post = Post::from_markdown(&content, slug);
            if post.meta.published {
                posts.push(post);
            }
        }
    }

    // need to render by handlebras
    let engine = TemplateEngine::new();
    for post in &posts {
        let html = engine.render_post(post);
        fs::write(format!("public/{}.html", post.slug), html).unwrap();
    }

    //
    let index_html = engine.render_index(&posts);
    fs::write("public/index.html", index_html).unwrap();
    Ok(())
}

fn clean_old_posts() -> Result<()> {
    let public_dir = Path::new("public");
    if public_dir.exists() {
        fs::remove_dir_all(public_dir)?;
    }
    fs::create_dir(public_dir).expect("can't create public dir");

    move_static_files()?;

    Ok(())
}

fn move_static_files() -> Result<()> {
    let static_dir = Path::new("static");
    let public_dir = Path::new("public/static");

    if !static_dir.exists() {
        return Ok(());
    }

    for entry in WalkDir::new(static_dir) {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            let relative_path = path.strip_prefix(static_dir).unwrap();
            let dest_path = public_dir.join(relative_path);

            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }

            fs::copy(path, dest_path)?;
        }
    }

    Ok(())
}
