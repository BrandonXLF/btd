# btd

## Build. Transform. Deploy.

`btd` allows you to transform and deploy production builds of projects with file operations and commands using easy-to-edit YAML files.

## Features

### Convenient File Format

Transformations are stored in [Instruction Files](https://brandonxlf.github.io/btd/file-format/) that are created using the easy-to-edit YAML file format. YAML makes it trivial to define multiline strings strings wile requiring minimal syntax.

### User-Wide Store

Instructions can be in codebase or in a [user-wide library](https://brandonxlf.github.io/btd/the-library/) that allows for the separation of open-source projects from the specific instructions for deploying them to proprietary environments.

## Installation

If you have Rust and Cargo installed ([install Rust](https://www.rust-lang.org/tools/install)), then you can build and install `btd` with the command `cargo install btd`.

On Windows, an [installer is available for download](`https://github.com/BrandonXLF/btd/releases/latest/download/btd-installer.exe`) that installs a precompiled copy of `btd` and adds it to the `$PATH`. A [precompiled exe](`https://github.com/BrandonXLF/btd/releases/latest/download/btd.exe) is also available directly.

## Documentation

You can find the `btd` documentation on [the website](https://brandonxlf.github.io/btd/).

## Developing

`btd` is build using Rust and Cargo. View [Cargo's documentation](https://doc.rust-lang.org/cargo/guide/working-on-an-existing-project.html) to learn more about developing with Cargo.
