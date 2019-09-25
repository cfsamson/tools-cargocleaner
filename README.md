# Cargo Clean for all subdirectories

`cargocleaner` simply walks all subdirectories from the current path it's called
from and if it finds a `Cargo.toml` document it runs the command `cargo clean`.

This can save huge amounts of space if you have many projects. One one of my
folders it went from 20 GB to 800 MB.

![cargo cleaner example](assets/cargocleaner_example.gif)

## Install

Clone this repository.

Run `cargo install --path ./`.

You should now be able to simply call `./cargocleaner` in a directory that 
contains one or more Cargo projects and let it run.

## Compatability

Currently it only works on `macos` and presumably `linux` but it's trivial
to make it work on Windows as well I just haven't needed it.

## Tip

This program will invoke commands on your system and `cargo clean` does delete files.
There is no logic in this code that deletes any thing. The source is very short so look
through it before running it on your system.