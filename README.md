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
HDPC Downloader version 1.4.0
Copyright 2020 b42-sneak; All rights reserved.
Licensed under the AGPL 3.0 <https://www.gnu.org/licenses/agpl-3.0.en.html>

HDPC downloader 1.4.0
b42-sneak <GitHub @b42-sneak>
Downloads comics from HDPC

USAGE:
    hdpc-dl [FLAGS] [OPTIONS] <URL>

FLAGS:
    -h, --help         Prints help information
    -j, --json-only    Only generate the JSON file
    -v                 Sets the level of verbosity: 1 for file names, 2 for percentage decimals
    -V, --version      Prints version information

OPTIONS:
    -d, --destination <destination>    Sets the download destination path [default: (the working directory)]

ARGS:
    <URL>    Sets the URL of the comic to download
```

### Examples

Download `https://example.com/1`:

- To the working directory: `hdpc-dl https://example.com/1`
- To `/some/other/directory`: `hdpc-dl https://example.com/1 -d /some/other/directory`

Only create (or overwrite) a JSON file `https://example.com/1`:

- In the working directory: `hdpc-dl https://example.com/1 -j`
- In `/some/other/directory`:
  - `hdpc-dl https://example.com/1 -d /some/other/directory -j`
  - `hdpc-dl https://example.com/1 -jd /some/other/directory`

### The `-v` flag

Here are some examples:

Without any (`hdpc-dl <URL>`):

```none
001/131 (  1%)
002/131 (  2%)
003/131 (  2%)
004/131 (  3%)
005/131 (  4%)
006/131 (  5%)
007/131 (  5%)
008/131 (  6%)
009/131 (  7%)
010/131 (  8%)
011/131 (  8%)
012/131 (  9%)
013/131 ( 10%)
014/131 ( 11%)
```

With one (`hdpc-dl <URL> -v`):

```none
Wrote file 001/131 (  1%): 001-comic-name-1.jpg
Wrote file 002/131 (  2%): 002-comic-name-2.jpg
Wrote file 003/131 (  2%): 003-comic-name-3.jpg
Wrote file 004/131 (  3%): 004-comic-name-4.jpg
Wrote file 005/131 (  4%): 005-comic-name-5.jpg
Wrote file 006/131 (  5%): 006-comic-name-6.jpg
Wrote file 007/131 (  5%): 007-comic-name-7.jpg
Wrote file 008/131 (  6%): 008-comic-name-8.jpg
Wrote file 009/131 (  7%): 009-comic-name-9.jpg
Wrote file 010/131 (  8%): 010-comic-name-10.jpg
Wrote file 011/131 (  8%): 011-comic-name-11.jpg
Wrote file 012/131 (  9%): 012-comic-name-12.jpg
Wrote file 013/131 ( 10%): 013-comic-name-13.jpg
Wrote file 014/131 ( 11%): 014-comic-name-14.jpg
```

With two or more (`hdpc-dl <URL> -vv` or `hdpc-dl <URL> -vvvvv`):

```none
Wrote file 001/131 (0.763%): 001-comic-name-1.jpg
Wrote file 002/131 (1.527%): 002-comic-name-2.jpg
Wrote file 003/131 (2.290%): 003-comic-name-3.jpg
Wrote file 004/131 (3.053%): 004-comic-name-4.jpg
Wrote file 005/131 (3.817%): 005-comic-name-5.jpg
Wrote file 006/131 (4.580%): 006-comic-name-6.jpg
Wrote file 007/131 (5.344%): 007-comic-name-7.jpg
Wrote file 008/131 (6.107%): 008-comic-name-8.jpg
Wrote file 009/131 (6.870%): 009-comic-name-9.jpg
Wrote file 010/131 (7.634%): 010-comic-name-10.jpg
Wrote file 011/131 (8.397%): 011-comic-name-11.jpg
Wrote file 012/131 (9.160%): 012-comic-name-12.jpg
Wrote file 013/131 (9.924%): 013-comic-name-13.jpg
Wrote file 014/131 (10.687%): 014-comic-name-14.jpg
```
