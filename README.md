# hdpc-dl

A downloader for HDPC.

Crawls a specified URL, writes all images and a JSON file to the working directory, or another directory if specified. If the directory does not already exist, its creation will be attempted.

Notice: The crawler was built specifically for HDPC and will not work for any other website. `example.com` (as shown below) would not work.

## Usage

The application is very easy to use, it only takes a URL, and an optional destination path (defaults to the working directory).

### Guide

1. `cargo install hdpc-dl`
2. `hdpc-dc https://example.com/target_url -d /home/b42-sneak/Downloads/HDPC`
3. A folder will be created: `/home/b42-sneak/Downloads/HDPC/Name-of-the-target`
4. In this folder ([...] `Name-of-the-target`) a JSON file will be created including all target metadata called `hdpc-info.json`
5. All images will be downloaded into the same directory, prefixed with an incrementing counter starting at `001-`

### The `--help` output

```none
╭─b42-sneak@b42-sneak-pc ~/src/target/release
╰─➤  hdpc-dl --help
HDPC Downloader version 1.0.0
Copyright 2020 b42-sneak; All rights reserved.
Licensed under the AGPL 3.0 <https://www.gnu.org/licenses/agpl-3.0.en.html>

HDPC downloader 1.0.0
b42-sneak <GitHub @b42-sneak>
Downloads comics from HDPC

USAGE:
    hdpc-dl [FLAGS] [OPTIONS] <URL>

FLAGS:
    -h, --help       Prints help information
    -v               Sets the level of verbosity
    -V, --version    Prints version information

OPTIONS:
    -d, --destination <destination>    Sets the download destination path [default: (the working directory)]

ARGS:
    <URL>    Sets the URL of the comic to download
```

Notice: The `-v` flag does nothing 🤷
