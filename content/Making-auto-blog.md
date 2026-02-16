---
title: "Making a auto blog with rust"
published: true
description: "This article primarily introduces how to i use rust to automate the conversion of a MarkDown files into html and deploy them to my blog site"
author: "MichaelScofield"
cover: "/static/img/auto-blog.png"
---

# Making a auto blog with rust

his article primarily introduces how to i use rust to automate the conversion of a MarkDown files into html and deploy them to my blog site

## Reinventing the wheel

I had one overarching goal; an easy experience, without sacrificing hackability. In the sprite of "easy authoring", what i wanted was a framework that would allow me to wirte a `.md` file, and push it to the GitHub, and have the result be published without any additional work.

while i did reinvent some of the wheel, i also avoiding reinventing as much of it as possible. I used an existing and excellent markdown parser called [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) to parse the markdown file, and then used Rust version of [Handlebars](https://crates.io/crates/handlebars) for template. one of the features pulldown dosen't have is frontmatter support, but luckily you can use markdown-frontmatter, it can easy to create a very simple parser to separate the metadata.Building things yourself is okay, but building your own buggy Markdown parser, instead of using an existing one that does what you need, is a great way to make sure you never make any progress on writing posts for your Markdown powered blog.

## Why choose Rust?

If you haven't used Rust in the last couple years, i would highly recommend giving it a chance. It's incredibly fast, and easy to get off the ground with, which is something i really value.It's also just a fantastic language. There's good reason for its consistently high rankings on StackOverflow surveys, and for all of the evangelists out there. Rust doesn't actually have very many ideas or features that are new or unique. But it mixes all of its parts together in a way that feels cohesive and magical. It is so much more than the sum of its parts, and it manages to maintain the fun of programming for me in a way that makes other languages feel dull in comparison.

Wanna try out the framework I made as a part of this blog? It's called BlogWing, right? it is a cool name and I'd love to hear about anything you make with it!
