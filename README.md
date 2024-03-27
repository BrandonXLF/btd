# btd

## Build. Transform. Deploy.

`btd` allows you to transform and deploy production builds of projects with file operations and commands using easy-to-edit YAML files.

## Features

### Convenient File Format

Transformations are stored in [Instruction Files](https://brandonxlf.github.io/btd/file-format/) that are created using the easy-to-edit YAML file format. YAML makes it trivial to define multiline strings strings wile requiring minimal syntax.

### User-Wide Store

Instructions can be in codebase or in a [user-wide library](https://brandonxlf.github.io/btd/the-library/) that allows for the separation of open-source projects from the specific instructions for deploying it to proprietary environments.

## Documentation

You can find the `btd` documentation on [the website](https://brandonxlf.github.io/btd/).

## Developing

`btd` is build using Rust and Cargo. View [Cargo's documentation](https://doc.rust-lang.org/cargo/guide/working-on-an-existing-project.html) to learn more about developing with Cargo.
