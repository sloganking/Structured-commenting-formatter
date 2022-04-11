use colored::Colorize;
use std::{env, path::PathBuf};
use strfmt::strfmt;

fn print_err(err: &str) {
    println!("{}: {}", "error".red().bold(), err);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        print_err("strfmt was passed too few arguments");
        return;
    } else if args.len() == 2 {
        let dir = &args[1];

        if dir == "help" {
            println!(
                "strfmt - structured commenting formatter

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
                let paths = strfmt::get_files_in_dir(dir, "");
                for file in paths {
                    strfmt::format_file(file);
                }
            } else if path.is_file() {
                strfmt::format_file(path);
            } else {
                print_err("arg must be a file, path, or command");
                return;
            }
        }
    } else if args.len() == 3 {
        let flag = &args[1];

        if flag == "add_brackets" || flag == "ab" {
            let dir = &args[2];
            let path = PathBuf::from(dir);

            if path.is_dir() {
                let paths = strfmt::get_files_in_dir(dir, "");
                for file in paths {
                    strfmt::add_brackets_file(file);
                }
            } else if path.is_file() {
                strfmt::add_brackets_file(path);
            } else {
                print_err("second arg must be a path or file");
                return;
            }
        } else if flag == "remove_brackets" || flag == "rb" {
            let dir = &args[2];
            let path = PathBuf::from(dir);

            if path.is_dir() {
                let paths = strfmt::get_files_in_dir(dir, "");
                for file in paths {
                    strfmt::remove_brackets_file(file);
                }
            } else if path.is_file() {
                strfmt::remove_brackets_file(path);
            } else {
                print_err("second arg must be a path or file");
                return;
            }
        } else if flag == "null" || flag == "n" {
            let dir = &args[2];
            let path = PathBuf::from(dir);

            if path.is_dir() {
                let paths = strfmt::get_files_in_dir(dir, "");
                for file in paths {
                    strfmt::null_existing_brackets_file(file);
                }
            } else if path.is_file() {
                strfmt::null_existing_brackets_file(path);
            } else {
                print_err("second arg must be a path or file");
                return;
            }
        } else {
            print_err("Invalid flag passed as arg");
            return;
        }
    } else {
        print_err("strfmt was passed too many arguments");
        return;
    }
}
