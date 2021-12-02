# gene-extractor

A command line app to extract gene from genomic contigs. The program requires phyluce and its dependencies to run. Future update may interact directly with Lastz or BWA-MEM or both.

## Installation

`genx` is a single executable command line app. The executable file will be available in the release [link](https://github.com/hhandika/myte/releases). Copy it to the folder that is registered in your PATH variable.

You can also install the app using the Rust package manager [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html):

```Bash
cargo install genx
```

OS support:

1. MacOS
2. Linux
3. Windows-WSL

Dependencies:

Phyluce and its depedencies ([install here](https://phyluce.readthedocs.io/en/latest/)).

## Usages

Subcommand options:

```Bash
USAGE:
    genx <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    check      Check dependencies
    extract    Batch gene tree estimation using IQ-Tree
    help       Prints this message or the help of the given subcommand(s)
```

Command options extract the gene:

```Bash
USAGE:
    genx extract --dir <PATH> --output <PATH> --refs <PATH>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --dir <PATH>       Path to contig directory
    -o, --output <PATH>    Path to output files
    -r, --refs <PATH>      Path to reference fasta files
```

