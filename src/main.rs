use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() <= 1 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "echo" => echo(args),
        "cat" => cat(args),
        _ => print_help(),
    }
}

fn print_help() {
    println!("Help here!");
}

fn echo(args: Vec<String>) {
    if args.len() <= 2 {
        return;
    }

    let mut output = String::new();
    for (i, arg) in args.iter().enumerate() {
        if i < 2 {
            continue;
        }

        output.push_str(arg);
        output.push(' ');
    }

    println!("{}", output);
}

fn cat(args: Vec<String>) {
    if args.len() < 2 {
        return;
    }

    let file_path = Path::new(&args[2]);

    if !Path::is_file(file_path) {
        println!("Invalid path!");
        return;
    }

    let file = File::open(&file_path);
    let mut file = match file {
        Ok(file) => file,
        Err(e) => return println!("A error occured by opening the file. Error: {}", e),
    };

    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .expect("Could not load File!");

    for (i, line) in file_contents.lines().enumerate() {
        if i > 128 {
            break;
        }

        println!("{}", line);
    }
}

fn find(args: Vec<String>) {
    if args.len() < 2 {
        return;
    }

    let start_path = fs::read_dir(&args[2]);
    let start_path = match start_path {
        Ok(start_path) => start_path,
        Err(e) => return println!("An error occured! Error: {}", e),
    };
}

fn grep(args: Vec<String>) {}
