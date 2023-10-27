use std::fs::{self, File, ReadDir};
use std::io::{self, Read};
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
        "find" => find(args),
        "grep" => grep(args),
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
    if args.len() < 4 {
        return;
    }

    fn get_readdir(start_path: &Path) -> io::Result<ReadDir> {
        return fs::read_dir(start_path);
    }

    fn search_directory(paths: ReadDir, search_target: &String) -> Vec<String> {
        let mut results: Vec<String> = Vec::new();
        for (_, path) in paths.enumerate() {
            let path = path.unwrap().path();

            if path
                .clone()
                .file_name()
                .unwrap()
                .to_os_string()
                .into_string()
                .unwrap()
                .to_lowercase()
                .contains(search_target)
            {
                results.push(path.display().to_string());
            }

            if path.is_dir() {
                let rdir = get_readdir(&path);
                let rdir = match rdir {
                    Ok(rdir) => rdir,
                    Err(_) => continue,
                };

                let directory_result = search_directory(rdir, search_target);
                for result in directory_result {
                    results.push(result);
                }
            }
        }

        return results;
    }

    fn display_result(results: Vec<String>) {
        for (_, str) in results.iter().enumerate() {
            println!("{}", str);
        }
    }

    let start_path = Path::new(&args[2]);
    let search_target = String::from(&args[3]).to_lowercase();

    if start_path.is_dir() {
        let results: Vec<String> =
            search_directory(fs::read_dir(start_path).unwrap(), &search_target);
        display_result(results);
    } else {
        println!("You provided a file. This command only supports directories. Use \"grep\" to search in a file!");
        return;
    }
}

fn grep(args: Vec<String>) {
    if args.len() < 4 {
        return;
    }

    let search_file = Path::new(&args[2]);
    let search_target = String::from(&args[3].to_lowercase());

    let file = File::open(search_file);
    let mut file = match file {
        Ok(file) => file,
        Err(_) => return println!("Could not open file!"),
    };

    let mut file_contents: String = String::new();
    let file_read_result = file.read_to_string(&mut file_contents);
    match file_read_result {
        Ok(file_read_result) => file_read_result,
        Err(_) => return println!("Could not read file!"),
    };

    let lowercase_content = file_contents.to_lowercase();
    let matches: Vec<_> = lowercase_content
        .match_indices(search_target.as_str())
        .collect();

    if matches.len() == 0 {
        println!("No matches found!");
        return;
    }

    for curr_match in matches {
        println!("Match on character {}", curr_match.0);
    }
}
