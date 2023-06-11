# ynab-csv

`ynab-csv` is a command line tool for transforming CSV bank statements of
Finnish banks into a form that can be imported to [YNAB](https://ynab.com).

Currently supported banks are S-Pankki and Danske Bank.

## Installation

Copy this repository. [Install Rust](https://www.rust-lang.org/tools/install).

In the root of this repository, run

```shell
$ cargo install --path .
```

## Usage

```shell
$ ynab-csv -h
Usage: ynab-csv --input <INPUT> --output <OUTPUT> --bank <BANK>

Options:
  -i, --input <INPUT>    Path to input CSV
  -o, --output <OUTPUT>  Path to output CSV
  -b, --bank <BANK>      Bank: "s-pankki" or "danske-bank"
  -h, --help             Print help
```