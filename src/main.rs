use colored::Colorize;
use scfmt::scfmt;
use std::{env, path::PathBuf};

fn print_err(err: &str) {
    println!("{}: {}", "error".red().bold(), err);
}

fn display_if_err(err_result: Result<(), (usize, String)>, file: PathBuf) {
    if let Err(err) = err_result {
        if err.1 != "Incompatible file type" {
            println!("{}: {}", "error".red().bold(), err.1);
            println!(
                "{}",
                file.as_os_str().to_str().unwrap().to_owned() + ":" + &format!("{}", err.0)
            );
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        print_err("scfmt was passed too few arguments");
    } else if args.len() == 2 {
        let dir = &args[1];

        if dir == "help" {
            println!(
"scfmt - structured commenting formatter

USAGE:
    [OPTIONS] [DIRECTORY]

OPTIONS:
    *None*                      Passing no option simply formats bracketed structured comments
    ab, add_brackets            Gives brackets to any bracketless strucutered comments
    rb, remove_brackets         Removes brackets from any bracketed structured comments
    n,  null                    Invalidates any existing bracketed comments, while preserving their content"
            )
        } else {
            let path = PathBuf::from(dir);

            if path.is_dir() {
                let paths = scfmt::get_files_in_dir(dir, "");
                for file in paths {
                    display_if_err(scfmt::format_file(file.to_path_buf()), file.to_path_buf());
                }
            } else if path.is_file() {
                display_if_err(scfmt::format_file(path.to_path_buf()), path);
            } else {
                print_err("arg must be a file, path, or command");
            }
        }
    } else if args.len() == 3 {
        let flag = &args[1];

        if flag == "add_brackets" || flag == "ab" {
            let dir = &args[2];
            let path = PathBuf::from(dir);

            if path.is_dir() {
                let paths = scfmt::get_files_in_dir(dir, "");
                for file in paths {
                    display_if_err(
                        scfmt::add_brackets_file(file.to_path_buf()),
                        file.to_path_buf(),
                    );
                }
            } else if path.is_file() {
                display_if_err(scfmt::add_brackets_file(path.to_path_buf()), path);
            } else {
                print_err("second arg must be a path or file");
            }
        } else if flag == "remove_brackets" || flag == "rb" {
            let dir = &args[2];
            let path = PathBuf::from(dir);

            if path.is_dir() {
                let paths = scfmt::get_files_in_dir(dir, "");
                for file in &paths {
                    display_if_err(
                        scfmt::remove_brackets_file(file.to_path_buf()),
                        file.to_path_buf(),
                    );
                }
            } else if path.is_file() {
                display_if_err(scfmt::remove_brackets_file(path.to_path_buf()), path);
            } else {
                print_err("second arg must be a path or file");
            }
        } else if flag == "null" || flag == "n" {
            let dir = &args[2];
            let path = PathBuf::from(dir);

            if path.is_dir() {
                let paths = scfmt::get_files_in_dir(dir, "");
                for file in paths {
                    scfmt::null_existing_brackets_file(file);
                }
            } else if path.is_file() {
                scfmt::null_existing_brackets_file(path);
            } else {
                print_err("second arg must be a path or file");
            }
        } else {
            print_err("Invalid flag passed as arg");
        }
    } else {
        print_err("scfmt was passed too many arguments");
    }
}
