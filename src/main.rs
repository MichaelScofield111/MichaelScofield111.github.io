use anyhow::{Ok, Result};

mod frontmatter;
mod generator;
mod post;
mod template;

fn main() -> Result<()> {
    generator::build()?;
    Ok(())
}
