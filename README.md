# 🦀 BlogWing — Personal Auto Blog Assistant

<p align="center">
<picture>
<img src="/assest/logo.jpg" alt="BlogWing logo" width="500">
</picture>

</p>

---

BlogWing is tiny project to build own your blog. then you can only write markdown then blogwing can simply auto to generate your blog and publish it to github pages.

## Features

- Write content in Markdown with YAML frontmatter
- Automatic HTML generation using Handlebars templates
- Support for tags, categories, and custom metadata
- Clean static output ready for GitHub Pages deployment

## Quick Start

```bash
# Build the blog
cargo build --release

# Run the generator
cargo run

# Output will be in public/ directory
```

## Project Structure

```
blogwing/
├── content/          # Your markdown blog posts
├── public/           # Generated HTML output (gitignored)
├── src/
│   ├── main.rs       # Entry point
│   ├── generator.rs  # Build logic
│   ├── post.rs       # Post model
│   ├── template.rs   # Template engine
│   └── frontmatter.rs # Metadata parsing
└── Cargo Writing.toml
```

## Posts

Create markdown files in `content/` directory with frontmatter:

```yaml
---
title: "Making a auto blog with rust"
published: true
description: "This article primarily introduces how to i use rust to automate the conversion of a MarkDown files into html and deploy them to my blog site"
author: "MichaelScofield"
cover: "/static/img/auto-blog.png"
---
```

## Tech Stack

- **Rust** - Programming language
- **handlebars** - Template engine
- **pulldown-cmark** - Markdown to HTML
- **markdown-frontmatter** - YAML frontmatter parsing
- **serde** - Serialization
