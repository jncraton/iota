# Iota [![Build Status](https://travis-ci.org/jncraton/iota.svg?branch=master)](https://travis-ci.org/jncraton/iota)

[![Gitter](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/gchp/iota?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

A terminal-based text-editor written in Rust.

## Motivation

This project is a fork of [Iota](https://github.com/ghcp/iota).

The editor should be light-weight, simple, and fit easily into a Unix workflow.

## Building

Clone the project and run `cargo build --release`.

## Usage

To start the editor run `./target/release/iota /path/to/file.txt`. Or
simply `./target/release/iota` to open an empty buffer.

You can also create buffers from `stdin`.

```bash
# open a buffer with the output of `ifconfig`
ifconfig | ./target/release/iota
```

You can move the cursor around with the arrow keys.

The following keyboard bindings are also available:

- `Ctrl-s` save
- `Ctrl-q` quit
- `Ctrl-k` delete current line
- `Ctrl-d` duplicate current line
- `Ctrl-x` cut current line
- `Ctrl-c` copy current line
- `Ctrl-v` paste
- `Ctrl-z` undo
- `Ctrl-y` redo
- `Ctrl-Left` move one word left
- `Ctrl-Right` move one word right
- `Ctrl-Up` move this line up
- `Ctrl-Down` move this line down
