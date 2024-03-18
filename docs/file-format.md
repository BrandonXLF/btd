# The Transform Deploy Instruction File Format

Instruction Files are YAML files consisting of a list of [Transformations](#transformations). To learn more about the YAML file format, visit [yaml.org](https://yaml.org/).

## Transformations

Each Transformation is a YAML dictionary with a `type` key corresponding to one of the options below.

### `meta`

The first entry. Contains information about the recipe.

* `dir` - Base directory to use for commands and file operations.

### `run`

Run a command.

* `cmd` - The command to run.

### `create`

Create a file with content.

* `file` - The file to create.
* `text` - Text to create the file with.

### `replace`

Find and replace text in a file.

* `file` - The file to operate on.
* `find` - The text to find.
* `replace` - The text to replace.
* `regex` - Boolean indicating of `find` should be interpreted as a regex expression.

### `prepend`

Find and replace text in a file.

* `file` - The file to create.
* `text` - Text to create the file with.

### `append`

Append text to the end of a file.

* `file` - The file to create.
* `text` - Text to create the file with.

### `rename`

Rename a file.

* `from` - The old file path.
* `to` - The new file path.

### `copy`

Copy a file.

* `from` - The path of the original file.
* `to` - The path of the copy to create.

### `delete`

Delete a file.

* `file` - The file to delete.

### `deploy`

Deploy a file to a production environment via secure copy.

* `from` - The local directory/file to copy from.
* `to` - The production directory/file to copy to as an `scp` path.
* `clear` - Boolean indicating of `to` should be removed first.
