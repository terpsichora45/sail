# Sail
## Description
A C package manager and directory supervisor. Sail is designed the perfect the project creation of C as well as the dependency management and library integration. Some other quality-of-life features will be added; such as: building and/or running, header creation, and makefile creation.

## Index
- [Sail](#sail)
  - [Description](#description)
  - [Index](#index)
  - [Current Focus](#current-focus)
- [Features](#features)
  - [List of Available Commands](#list-of-available-commands)
- [Authors](#authors)

## Current Focus
[Operating File](https://github.com/terpsichora45/sail/blob/dev/src/bin/test.rs)<br>
I am currently working on the command creation system using Rust macros to automatically structure a command vector that stores the command information for easier checking and new command integration.

# Features
The [command list](#list-of-available-commands) also displays the currently WIP aspects of Sail.
## List of Available Commands
* `help`: displays the command list and related information.
* `init`: creates the directories and files necessary for a new C project. (awaiting makefile boilerplate)
* `build`: compiles C files into object files and then into a binary executable using GCC. (awaiting integration)
* `run`: if created, executes the binary. otherwise, creates and *then* executes the binary. (awaiting integration)
* `new-header`: creates a new header file within the project directory in the proper location. (awaiting integration)

# Authors
[Terpsichora](https://github.com/terpsichora45/)
