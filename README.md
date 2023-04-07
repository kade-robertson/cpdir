# cpdir

A utility for copying directory structures between two locations.

## Info

This will only copy directories, no files will be moved or created.

Any errors preventing a single directory from being moved only mean that single
directory will not be moved.

## Usage

```
Command-line utility for copying directory structure between two locations.

Usage: cpdir [OPTIONS] <SOURCE> <DEST>

Arguments:
  <SOURCE>  The directory to walk to determine the structure to be copied to the
            destination
  <DEST>    The directory to copy walked paths into

Options:
  -d, --depth <DEPTH>  Controls the recursion depth of the directory walking
                       process. The allowed range is 0-255. Setting this to 0
                       will effectively not walk the source directory [default:
                       255]
  -x, --execute        Controls whether folders should actually be created at
                       the destination. This defaults to false, so this program
                       will run in a "dry-run" mode unless this argument is
                       specified
      --allow-nesting  Whether or not the destination directory is allowed to be
                       a subdirectory of the source. This can have undesired
                       behaviour, as directories within the subdirectory then
                       get copied deeper into the same subdirectory
  -h, --help           Print help
  -V, --version        Print version
```
