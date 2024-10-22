# Running Instructions

[Instruction files](/file-format) are run with the [`btd`](/commands/) command via `btd [<name>]`. If no `<name>` is specified, the script in [the Library](/the-library/) with its meta `dir` set to the current directory will be run. Otherwise, the `<name>` script will be run if it exists, and if it doesn't, the script with the name `<name>` in [the Library](/the-library/) will be run. `.yml` is added to `<name>` as required.
