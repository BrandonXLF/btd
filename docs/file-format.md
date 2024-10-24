# File Format

**Instruction files** are YAML files consisting of a list of [transformations](#transformations). To learn more about the YAML file format, visit [yaml.org](https://yaml.org/).

## Transformations

Each transformation is a YAML dictionary with a `type` key corresponding to one of the options below.

### `meta`

The first entry. Contains information about the instruction file.

* `dir` (*recommended*) string - Directory of the codebase for the instruction file. Used as the base directory for commands and file operations. If this path is relative, [the Library](/the-library/)'s base is used as the base for this path. If omitted or blank, defaults to the Library's base if part of the Library of the current default directory. Recommended for files in the Library.

### `run`

Run a command.

* `cmd` string - The command to run.
* `cwd` (*optional*) string - The current working directory to run the command in. Defaults to the `meta` transformation's `dir`.
* `env` (*optional*) string: string map - Mapping of environment variables to set.

### `create`

Create a file with content.

* `file` string - The file to create.
* `text` string - Text to create the file with.

### `replace`

Find and replace text in a file.

* `file` string - The file to replace text in.
* `find` string - The text to find. Interpreted as normal text unless `regex` is `true`.
* `replace` string - The text to replace. If `regex` is `true`, substitutions (eg. `$1`, `$2`, etc.) are supported.
* `regex` (*optional*) boolean - Interpret `find` as a regular expression. Default is `false`.

### `prepend`

Find and replace text in a file.

* `file` string - The file to prepend text to.
* `text` string - Text to prepend the file with.

### `append`

Append text to the end of a file.

* `file` string - The file to append to text.
* `text` string - Text to append the file with.

### `rename`

Rename a file or directory.

* `from` string - The old file path.
* `to` string - The new file path.

### `copy`

Copy a file. To copy a directory, use `scp`.

* `from` string - The path of the original file.
* `to` string - The path of the copy to create.

### `delete`

Delete a file or directory.

* `file` string - The file or directory to delete.
* `recursive` (*optional*) boolean - Delete items recursively if `file` is a directory. Default is `false`.

### `deploy`

Deploy a file or directory to a production environment via secure copy.

* `from` string - The local directory/file to copy from.
* `to` string - The production directory/file to copy to as an `scp` path.
* `clear` (*optional*) boolean - Clear the contents of `to` before copying files from `from`. Files in `from` are copied to the sever beforehand so the copy into `to` is a local copy. Can only be used with directories.
