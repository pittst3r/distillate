# Distillate

A minimalist markdown to html static site generator.

This is a personal project to practice Rust and motivate myself to make a personal site and blog. Pretty sure we don't actually need another static site generator!

## Usage

```
distillate build <destination directory path; will be generated if necessary>
```

### Installation

This is a Rust crate that is unpublished and will probably remain so. One may install it from source on GitHub:

```sh
cargo install --git https://github.com/pittst3r/distillate.git
```

### Conventions

- Put markdown files in `src` directory in your project

## To-do

- [x] Make page template editable in userspace
- [x] Extract page title from `h1` since `pulldown-cmark` doesn't support frontmatter
- [ ] Generate table of contents for homepage, perhaps using hbs for insertion
