# The Library

The Library is a user-wide collection of [instruction files](/file-format/). It allows for storing instruction files outside of project directories. This allows for the separation of open-source projects and the specific instructions from the specific instructions for deploying them to proprietary environments.

## Managing

Files in the Library are managed with the [`btd`](/commands/) command.

## Location

The [`btd`](/commands/) can be used to set the default location to store instruction files in the Library. This allows the storing of files in any directory, including ones that are synced across computers.

## Base Directory for Codebase Directories

[Instruction files](/file-format/) in the Library have a directory that corresponds to the codebase they are for. This directory can be relative, in which case the Library's base is used. This allows [instruction files](/file-format/) to be used on computer with different locations for storing codebases. The default base is the current working directory.
