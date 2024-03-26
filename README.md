# Transform Deploy

<div style="font-size: 1.35rem">Production build to deployment file transformer</div>

---

Transform production builds of projects using file operations and commands into a deployment-ready state forms and deploy them.

## Features

### Convenient File Format

Transformations are stored in [Instruction Files](https://brandonxlf.github.io/tdep/file-format) that are created using the easy to edit YAML file format. YAML makes it trivial to define multiline strings strings wile requiring minimal syntax.

### Computer-Wide Store

Instructions can be in codebase or in a [computer-wide library](https://brandonxlf.github.io/tdep/the-library) that allows for the separation of open-source projects from the specific instructions for deploying it to proprietary environments.

## Documentation

You can find the Transform Deploy documentation on [the website](https://brandonxlf.github.io/tdep/).

## Developing

Transform Deploy is build using Rust and Cargo. View [Cargo's documentation](https://doc.rust-lang.org/cargo/guide/working-on-an-existing-project.html) to learn more about developing with Cargo.
