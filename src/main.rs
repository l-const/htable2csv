extern crate isahc;
extern crate nipper;

use isahc::prelude::*;
use nipper::Document;
use nipper::Selection;
use std::fs::File;
use std::io::{BufWriter, Write};

type Table = Vec<Vec<String>>;

fn parse_document(doc: Document) {
    let selected_table_iter: Vec<_> = doc.select("table").iter().collect();
    for (pos, item) in selected_table_iter.into_iter().enumerate() {
        parse_table(pos, item);
    }
}

fn parse_table(pos: usize, sel: Selection) {
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
    write_to_file(pos, vec_of_vec).expect("Error creating file!");
}

fn write_to_file(pos: usize, table: Table) -> std::io::Result<()> {
    let filename = format!("out{}.csv", pos);
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

fn get_page(page: &str) -> () {
    let mut response = isahc::get(page).expect("Error in http get");
    if let Ok(body) = response.text() {
        let document = Document::from(&body);
        parse_document(document)
    } else {
        println!("Error geting response body!");
    }
}

fn main() {
    let mut args = std::env::args();
    if let Some(page) = args.nth(1) {
        get_page(&page);
    } else {
        println!("Not enought arguments!");
    }
}
