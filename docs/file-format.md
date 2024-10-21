# File Format

**Instruction files** are YAML files consisting of a list of [Transformations](#transformations). To learn more about the YAML file format, visit [yaml.org](https://yaml.org/).

## Transformations

Each Transformation is a YAML dictionary with a `type` key corresponding to one of the options below.

### `meta`

The first entry. Contains information about the instruction file.

* `dir` string - Base directory to use for commands and file operations. All relatives paths are processed relative to this path.

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
* `clear` (*optional*) boolean - Remove `to` before replacing it with `from`. Useful for directories. Files are transferred to the remote server before `to` is removed. Default is `false`.
