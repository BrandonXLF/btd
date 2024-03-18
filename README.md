# `tdep` - Transform Deploy

Create and deploy production builds of projects (apps, websites, etc.) using recipes of file transformation and shell commands using convenient YAML [Instruction Files](#instruction-files) files.

## Instruction Files

An Instruction File is a YAML file containing a list of Transformations to perform for a project. Read [The Transform Deploy Instruction File Format](docs/file-format.md) to learn about the format.

### Running Instruction Files

Instruction Files are run with the `tdep [<name>]` command. If no `<name>` is specified, the script in [The Library](#the-library) with its meta `dir` set to the current directory will be run. Otherwise, the `<name>` script will be run if it exists, and if it doesn't, the script with the name `<name>` in [The Library](#the-library) will be run. `.yml` is added to `<name>` as required.

## The Library

The Library is a computer-wide collection of recipes. It allows for storing transformation recipes outside of project directories. This allows for the separation of the, potentially open-source, project and specific recipes for deploying it to proprietary environments.

The Library can be managed with the `tdep <--create | --delete | --edit | --list | --open | --rename>` commands.
