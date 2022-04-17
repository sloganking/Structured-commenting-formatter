use colored::Colorize;
use scfmt::scfmt;
use std::{env, path::PathBuf};
#[macro_use]
extern crate version;

fn print_err(err: &str) {
    println!("{}: {}", "error".red().bold(), err);
}

fn display_if_err(err_result: Result<(), (usize, String)>, file: PathBuf) {
    if let Err(err) = err_result {
        if err.1 != "Incompatible file type" && err.1 != "Cannot determine file extension" {
            println!("{}: {}", "error".red().bold(), err.1);
            let mut line_string = "".to_owned();
            if err.0 != 0 {
                line_string = ":".to_owned() + &format!("{}", err.0)
            }
            println!(
                "{}",
                file.as_os_str().to_str().unwrap().to_owned() + &line_string
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
    n,  null                    Invalidates any existing bracketed comments, while preserving their content
    v,  version                 Print current version info"
            )
        } else if &args[1] == "version" || &args[1] == "v" {
            println!("scfmt {}", version!());
        } else {
            let path = PathBuf::from(dir);

            if path.is_dir() {
                match scfmt::get_files_in_dir(dir, "") {
                    Ok(paths) => {
                        for file in paths {
                            display_if_err(
                                scfmt::format_file(file.to_path_buf()),
                                file.to_path_buf(),
                            );
                        }
                    }
                    Err(err) => println!("{:?}", err),
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
                match scfmt::get_files_in_dir(dir, "") {
                    Ok(paths) => {
                        for file in paths {
                            display_if_err(
                                scfmt::add_brackets_file(file.to_path_buf()),
                                file.to_path_buf(),
                            );
                        }
                    }
                    Err(err) => println!("{:?}", err),
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
                match scfmt::get_files_in_dir(dir, "") {
                    Ok(paths) => {
                        for file in &paths {
                            display_if_err(
                                scfmt::remove_brackets_file(file.to_path_buf()),
                                file.to_path_buf(),
                            );
                        }
                    }
                    Err(err) => println!("{:?}", err),
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
                match scfmt::get_files_in_dir(dir, "") {
                    Ok(paths) => {
                        for file in paths {
                            scfmt::null_existing_brackets_file(file);
                        }
                    }
                    Err(err) => println!("{:?}", err),
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
