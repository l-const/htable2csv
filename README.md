# htable2csv

[<img alt="crates.io" src="https://img.shields.io/crates/v/htable2csv.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/htable2csv)

Scrape HTML tables to .csv files, from your terminal.

#### Description: 
Program accepts an http url and downloads HTML tables 
contained as .csv files.

 Install (from crates.io)

```bash
cargo install htable2csv
```

 Run 

```bash
htable2csv https://www.w3schools.com/html/html_tables.asp
```
Help

```bash
htable2csv -h

htable2csv 0.1.1
Kostas L. <konlampro94@gmail.com>
Scrapes HTML tables from the web!

USAGE:
    htable2csv <uri> [ARGS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <uri>        Http page url to be scraped. (Required)
    <out_dir>    Relative path for the folder to place the output. (Optional)
    <prefix>     File prefix for the output files for each table. (Optional)

```

