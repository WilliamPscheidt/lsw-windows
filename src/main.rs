use std::{fs};
use ansi_term::Colour::Cyan;
use prettytable::{row, Table, format};

fn main() {
    list_files_in_path();
}

fn list_files_in_path() {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_titles(row!["lsw file list"]);
    table.add_row(row!["Size", "Archive"]);
    table.add_empty_row();
    for file in fs::read_dir(".").expect("Invalid path") {
        let path = file.unwrap().path();
        let path_filesize = path.clone();
        let metadata = fs::metadata(path_filesize).expect("Failed to get metadata");
        let file_size: u64 = metadata.len();
        table.add_row(row![file_size, Cyan.paint(path.display().to_string())]);
    }
    table.printstd();
}
