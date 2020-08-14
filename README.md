# hdpc-dl

A downloader for HDPC.

Crawls a specified URL, writes all images and a JSON file to the working directory, or another directory if specified. If the directory does not already exist, its creation will be attempted.

Notice: The crawler was built specifically for HDPC and will not work for any other website. `example.com` (as shown below) would not work.

## Usage

The application is very easy to use, it only takes a URL (in `one`-mode), and an optional destination path (defaults to the working directory).

### Guide

1. `cargo install hdpc-dl`
2. `hdpc-dc -d /home/b42-sneak/Downloads/HDPC one https://example.com/target_url`
3. A folder will be created: `/home/b42-sneak/Downloads/HDPC/Name-of-the-target`
4. In this folder ([...] `Name-of-the-target`) a JSON file will be created including all target metadata called `hdpc-info.json`
5. All images will be downloaded into the same directory, prefixed with an incrementing counter starting at `001-`

### The help outputs

#### `hdpc-dl --help`

```none
╭─b42-sneak@b42-sneak-pc ~/Downloads/hd-pc
╰─➤  hdpc-dl --help
HDPC Downloader 2.1.0
b42-sneak <GitHub @b42-sneak>
Downloads comics from HDPC

USAGE:
    hdpc-dl [FLAGS] [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help         Prints help information
    -j, --json-only    Only generate the JSON file
    -v                 Sets the level of verbosity: 1 for file names, 2 for percentage decimals
    -V, --version      Prints version information

OPTIONS:
    -d, --destination <destination>    Sets the download destination path [default: (the working directory)]

SUBCOMMANDS:
    crawl    Finds all comics on a URL and downloads them all
    one      Downloads one comic

Copyright 2020 b42-sneak; All rights reserved.
Licensed under the AGPL 3.0 <https://www.gnu.org/licenses/agpl-3.0.en.html>
```

#### `hdpc-dl one --help`

```none
╭─b42-sneak@b42-sneak-pc ~/Downloads/hd-pc
╰─➤  hdpc-dl one --help
hdpc-dl-one 2.1.0
Downloads one comic

USAGE:
    hdpc-dl one <URL>

FLAGS:
    -h, --help    Prints help information

ARGS:
    <URL>    Sets the URL of the comic to download

Copyright 2020 b42-sneak; All rights reserved.
Licensed under the AGPL 3.0 <https://www.gnu.org/licenses/agpl-3.0.en.html>
```

#### `hdpc-dl crawl --help`

(In development; not finalized)

```none
╭─b42-sneak@b42-sneak-pc ~/Downloads/hd-pc
╰─➤  hdpc-dl crawl --help
hdpc-dl-crawl 2.1.0
Finds all comics on a URL and downloads them all

USAGE:
    hdpc-dl crawl [FLAGS] [OPTIONS] <URL>

FLAGS:
    -h, --help      Prints help information
    -p, --paging    Tries to continue on the next page withing the download limit & offset

OPTIONS:
    -l, --limit <limit>    Limit to n finding(s) to be downloaded [default: 0]
    -s, --skip <skip>      Skip the first n finding(s) [default: 0]

ARGS:
    <URL>    Sets the URL of the page to be crawled

Copyright 2020 b42-sneak; All rights reserved.
Licensed under the AGPL 3.0 <https://www.gnu.org/licenses/agpl-3.0.en.html>
```

### Examples

Download `https://example.com/something`:

- To the working directory: `hdpc-dl one https://example.com/something`
- To `/some/other/directory`: `hdpc-dl one https://example.com/something -d /some/other/directory`

Only create (or overwrite) a JSON file for `https://example.com/something`:

- In the working directory: `hdpc-dl -j one https://example.com/something`
- In `/some/other/directory`:
  - `hdpc-dl -d /some/other/directory -j one https://example.com/something`
  - `hdpc-dl -jd /some/other/directory one https://example.com/something`

The `crawl` sub-command downloads everything it finds on a page (except for the page itself).  
You can limit it using the `--limit 42` (or `-l 42`) option to limit it to 42 targets.

When the `--paging` (or `-p`) flag is set, it will download other pages, until it has enough results.

### The `-v` flag

The `-v` flag is an argument of the main command (not of one of the sub-commands), and thus needs to be specified _before_ `one` or `crawl`.

Here are some examples:

Without any (`hdpc-dl one <URL>`):

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

With one (`hdpc-dl -v one <URL>`):

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

With two or more (`hdpc-dl -vv one <URL>` or `hdpc-dl -vvvvv one <URL>`):

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
