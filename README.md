# cpdir

![test results](https://img.shields.io/github/actions/workflow/status/kade-robertson/cpdir/test.yml?label=test&style=flat-square) ![coverage](https://img.shields.io/codecov/c/github/kade-robertson/cpdir?style=flat-square)
[![version](https://img.shields.io/crates/v/cpdir?style=flat-square)](https://crates.io/crates/cpdir) [![downloads](https://img.shields.io/crates/d/cpdir?style=flat-square)](https://crates.io/crates/cpdir)

A utility for copying directory structures between two locations.

## Info

This will only copy directories, no files will be moved or created.

Any errors preventing a single directory from being moved only mean that single
directory will not be moved. By default, this also means that the exit code from
running this application (aside from nested instances) will always be zero. If
you want to have the application report a non-zero exit code, use the `-E` flag.

## Usage

```
Command-line utility for copying directory structure between two locations.

Usage: cpdir [OPTIONS] <SOURCE> <DEST>

Arguments:
  <SOURCE>  The directory to walk to determine the structure to be copied to the
            destination
  <DEST>    The directory to copy walked paths into

Options:
  -d, --depth <DEPTH>      Controls the recursion depth of the directory walking
                           process. The allowed range is 0-255. Setting this to
                           0 will effectively not walk the source directory
                           [default: 255]
  -x, --execute            Controls whether folders should actually be created
                           at the destination. This defaults to false, so this
                           program will run in a "dry-run" mode unless this
                           argument is specified
      --allow-nesting      Whether or not the destination directory is allowed
                           to be a subdirectory of the source. This can have
                           undesired behaviour, as directories within the
                           subdirectory then get copied deeper into the same
                           subdirectory
  -E, --exit-code-failure  Whether or not a non-zero exit code should be used in
                           the case that any directory is not able to be moved
  -h, --help               Print help
  -V, --version            Print version
```
