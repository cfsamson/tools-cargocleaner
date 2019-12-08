# Cargo clean for all subdirectories

`cargocleaner` simply walks all subdirectories from the current path it's called
from and if it finds a `Cargo.toml` document it runs the command `cargo clean`.

It's the same as if you manually went through each folder and ran `cargo clean`
manually.

This can save huge amounts of space if you have many projects. One one of my
folders it went from 20 GB to 800 MB.

![cargo cleaner example](assets/cargocleaner_example.gif)

## Install

### Cargo install

`cargo install cargocleaner`

To update the program

`cargo install --force cargocleaner`

### Manually

Clone this repository.
Run `cargo install --path ./`.

You should now be able to simply call `./cargocleaner` in a directory that 
contains one or more Cargo projects and let it run.

## Limitations

Right now this is just a very simple tool but since we don't go through sub folders
if we find a `Cargo.toml` in the root. This should work fine with `workspaces` but
if for some reason you have a file called `Cargo.toml` which is not in the root of
a crate it might not continue down any subdirectories.

## Tip

This program will invoke commands on your system. `cargo clean` does delete files,
but there is no logic in this code can delete anything besides what `cargo clean`
does. The source is very short so look through it before running it on your system
if you're in doubt.
