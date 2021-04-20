extern crate clap;
extern crate isahc;
extern crate nipper;

use clap::{App, Arg};
use isahc::prelude::*;
use nipper::Document;
use nipper::Selection;

use std::fs::File;
use std::io::{BufWriter, Write};

type Table = Vec<Vec<String>>;

#[derive(Debug, Default)]
struct CliOpts<'cli_input> {
    page: &'cli_input str,
    out_dir: &'cli_input str,
    prefix: &'cli_input str,
}


fn parse_document(doc: Document, cli_opts: CliOpts) {
    let selected_table_iter: Vec<_> = doc.select("table").iter().collect();
    for (pos, item) in selected_table_iter.into_iter().enumerate() {
        parse_table(pos, item, &cli_opts);
    }
}

fn parse_table(pos: usize, sel: Selection, cli_opts: &CliOpts) {
    let vec_of_vec: Table;
    let rows = sel.select("tr");
    vec_of_vec = rows
        .iter()
        .map(|row| {
            row.iter()
                .map(|node| node.text().to_string().trim().into())
                .collect()
        })
        .collect();
    write_to_file(pos, vec_of_vec, cli_opts).expect("Error creating file!");
}

fn write_to_file(pos: usize, table: Table, cli_opts: &CliOpts) -> std::io::Result<()> {
    let filename = format!("{}{}{}.csv", cli_opts.out_dir, cli_opts.prefix , pos);
    let file = File::create(&filename).expect("Couldn't create file!");
    let mut buffer = BufWriter::new(file);
    for row in table {
        for item in row {
            let nodes: Vec<_> = item.split('\n').collect();
            let nodes: Vec<_> = nodes
                .iter()
                .map(|&str_r| str_r.trim().to_string())
                .collect();
            let mut line_to_write = nodes.join(",");
            line_to_write.push_str("\n");
            buffer.write(line_to_write.as_bytes())?;
        }
    }

    buffer.flush()?;
    Ok(())
}

fn get_page(cli_opts: CliOpts) {
    let mut response = isahc::get(cli_opts.page).expect("Error in http get");
    if let Ok(body) = response.text() {
        let document = Document::from(&body);
        parse_document(document, cli_opts)
    } else {
        println!("Error geting response body!");
    }
}

fn main() {
    let matches = App::new("htable2csv")
        .version("0.1.1")
        .author("Kostas L. <konlampro94@gmail.com>")
        .about("Scrapes HTML tables from the web!")
        .arg(
            Arg::with_name("uri")
                .required(true)
                .takes_value(true)
                .help("Http page url to be scraped. (Required)"),
        )
        .arg(
            Arg::with_name("out_dir")
                .takes_value(true)
                .help("Relative path for the folder to place the output. (Optional)"),
        )
        .arg(
            Arg::with_name("prefix")
                .takes_value(true)
                .help("File prefix for the output files for each table. (Optional)"),
        )
        .get_matches();
    let cli_opts = CliOpts {
        page : matches.value_of("uri").unwrap(),
        out_dir : matches.value_of("out_dir").unwrap_or("./"),
        prefix : matches.value_of("prefix").unwrap_or("out")
    };
    get_page(cli_opts);
}
