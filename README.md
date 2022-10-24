# Sail
## Description
A C package manager and directory supervisor. Sail is designed the perfect the project creation of C as well as the dependency management and library integration. Some other quality-of-life features will be added; such as: building and/or running, header creation, and makefile creation.

## Layout
* Develop the modularity of the system using command arguments
* Complete the error handling for the `unwrap` instances
* Start the `new-header` command

## Index
- [Sail](#sail)
  - [Description](#description)
  - [Layout](#layout)
  - [Index](#index)
  - [Current Focus](#current-focus)
- [Features](#features)
  - [List of Available Commands](#list-of-available-commands)
- [Authors](#authors)
  - [Terpsichora <img src="https://avatars.githubusercontent.com/u/63125641?v=4" style="border-radius:50%; width:70px; height:auto;">](#terpsichora-)
  - [Leddoo](#leddoo)

## Current Focus
[Operating File](https://github.com/terpsichora45/sail/blob/dev/src/main.rs)<br>
I am currently working on the command execution handling. I'm ensuring that error codes maintain some form of user-end abstraction while also providing enough information for debugging information for the average user.

# Features
The [command list](#list-of-available-commands) also displays the currently WIP aspects of Sail.
## List of Available Commands
* `help`: displays the command list and related information.
* `init`: creates the directories and files necessary for a new C project.
* `build`: compiles C files into object files and then into a binary executable using GCC.
* `run`: if created, executes the binary. otherwise, creates and *then* executes the binary.
* `new-header`: creates a new header file within the project directory in the proper location. (awaiting integration)

# Authors
## [Terpsichora](https://github.com/terpsichora45/) <img src="https://avatars.githubusercontent.com/u/63125641?v=4" style="border-radius:50%; width:70px; height:auto;">
## [Leddoo](https://github.com/leddoo/)
