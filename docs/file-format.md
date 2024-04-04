# File Format

Instruction Files are YAML files consisting of a list of [Transformations](#transformations). To learn more about the YAML file format, visit [yaml.org](https://yaml.org/).

## Transformations

Each Transformation is a YAML dictionary with a `type` key corresponding to one of the options below.

### `meta`

The first entry. Contains information about the Instruction File.

* `dir` - Base directory to use for commands and file operations. All relatives paths are processed relative to this path.

### `run`

Run a command.

* `cmd` - The command to run.
* `cwd` (*optional*) - The current working directory to run the command in. Defaults to the `meta` transformation's `dir`.
* `env` (*optional*) - Mapping of environment variables to set.

### `create`

Create a file with content.

* `file` - The file to create.
* `text` - Text to create the file with.

### `replace`

Find and replace text in a file.

* `file` - The file to replace text in.
* `find` - The text to find. Interpreted as normal text unless `regex` is `true`.
* `replace` - The text to replace. If `regex` is `true`, substitutions (eg. `$1`, `$2`, etc.) are supported.
* `regex` (*optional*) - Boolean indicating if `find` should be interpreted as a regex expression. Defaults to `false`.

### `prepend`

Find and replace text in a file.

* `file` - The file to prepend text to.
* `text` - Text to prepend the file with.

### `append`

Append text to the end of a file.

* `file` - The file to append to text.
* `text` - Text to append the file with.

### `rename`

Rename a file.

* `from` - The old file path.
* `to` - The new file path.

### `copy`

Copy a file.

* `from` - The path of the original file.
* `to` - The path of the copy to create.

### `delete`

Delete a file or directory.

* `file` - The file or directory to delete.
* `recursive` (*optional*) - Boolean indicating if items should be deleted recursively if `file` is a directory.

### `deploy`

Deploy a file to a production environment via secure copy.

* `from` - The local directory/file to copy from.
* `to` - The production directory/file to copy to as an `scp` path.
* `clear` (*optional*) - Boolean indicating if `to` should be removed and replaced with `from`. Files are transferred to the remote server before `to` is deleted. Defaults to `false`. 
