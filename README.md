# Distillate

A minimalist markdown to html static site generator.

## Usage

```sh
distillate build
```

### Installation

This is a Rust crate that is currently unpublished, but you may install it from source on GitHub to try it out:

```sh
cargo install --git https://github.com/pittst3r/distillate.git
```

### Conventions

- Put markdown files in `src` directory in your project
- HTML files are generated into a `dist` directory using the same file structure

## To-do

- [ ] Make page template editable in userspace
- [ ] Generate table of contents for homepage, perhaps using hbs for insertion
