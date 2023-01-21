use ansi_term::Colour::Cyan;
use prettytable::{format, row, Table};
use std::{fs, io};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "lsw", about = "lsw file manager")]
struct Opt {
    #[structopt(short = "l", long = "list", help = "List all files in current directory")]
    list: bool,
    #[structopt(short = "g", long = "graph", help = "List all files in current directory with a table")]
    graph: bool,
    #[structopt(short = "c", long = "create", help = "Create a new folder in the current directory")]
    create: bool,
    #[structopt(short = "s", long = "search", help = "Search for a folder")]
    search: bool,
}

fn main() {
    let opt = Opt::from_args();
    if opt.list {
        list_files_in_path();
    } else if opt.graph {
        list_files_in_path_with_graph();
    } else if opt.create {
        create_folder_in_path();
    } else if opt.search {
        search_file_in_path();
    } else {
        println!("Use lsw -l | lsw -g | lsw -c")
    }   
}

fn list_files_in_path_with_graph() {
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

fn search_file_in_path() {
    let mut folder_name_input: String = String::new();
    println!("{}", "$ Type your folder name: ");
    io::stdin().read_line(&mut folder_name_input).expect("Invalid folder name");
    for file in fs::read_dir(".").expect("Invalid path") {
        let path = file.unwrap().path().display().to_string().replace(".\"", "");
        if path.trim() == folder_name_input.trim().to_string() {
            println!("[+] {} found", folder_name_input.to_string().trim())
        }
    }
}

fn list_files_in_path() {
    for file in fs::read_dir(".").expect("Invalid path") {
        println!("{}", file.unwrap().path().display().to_string());
    }
}

fn create_folder_in_path() {
    let mut folder_name_input: String = String::new();
    println!("{}", "$ Type your folder name: ");
    io::stdin().read_line(&mut folder_name_input).expect("Invalid folder name");
    let folder_name = folder_name_input.clone();
    fs::create_dir(folder_name.trim().to_string()).expect("Error in folder creation");
}
