use ::scfmt::scfmt::ScfmtErr;
use colored::Colorize;
use scfmt::scfmt;
use std::{env, path::PathBuf};
#[macro_use]
extern crate version;

static HELP_STR: &str =
"scfmt - structured commenting formatter

USAGE:
    [OPTIONS] [DIRECTORY]

OPTIONS:
    *None*                      Passing no option simply formats bracketed structured comments
    ab, add_brackets            Gives brackets to any bracketless strucutered comments
    rb, remove_brackets         Removes brackets from any bracketed structured comments
    n,  null                    Invalidates any existing bracketed comments, while preserving their content
    v,  version                 Print current version info";

fn print_err(err: &str) {
    println!("{}: {}", "error".red().bold(), err);
}

fn print_if_err(err_result: Result<(), ScfmtErr>, file: PathBuf) {
    if let Err(err) = err_result {
        let file_string = match file.as_os_str().to_str() {
            Some(str) => str.to_owned(),
            None => format!("{:?}", file),
        };

        match err {
            ScfmtErr::CommentClosedNothing(line) => print_err(
                &("comment closed nothing\n".to_owned()
                    + &file_string
                    + ":"
                    + &format!("{}", line)),
            ),
            ScfmtErr::CommentNeverClosed(line) => print_err(
                &("comment never closed\n".to_owned() + &file_string + ":" + &format!("{}", line)),
            ),
            ScfmtErr::CantConvertOsString => {
                print_err(&("Cannot convert OS String to displayable\n".to_owned() + &file_string))
            }
            ScfmtErr::CantReadFileAsString => {
                print_err(&("Cannot read file as string\n".to_owned() + &file_string))
            }
            ScfmtErr::CantCreatFile => {
                print_err(&("Cannot create file\n".to_owned() + &file_string))
            }
            ScfmtErr::CantWriteToFile => {
                print_err(&("Cannot write to file\n".to_owned() + &file_string))
            }
            _ => {}
        }
    }
}

fn attempt_transform_path(f: fn(PathBuf) -> Result<(), ScfmtErr>, dir: &str) {
    let path = PathBuf::from(dir);

    if path.is_dir() {
        match scfmt::get_files_in_dir(dir, "") {
            Ok(paths) => {
                for file in paths {
                    print_if_err(f(file.to_path_buf()), file.to_path_buf());
                }
            }
            Err(err) => println!("{}: {}", "error".red().bold(), err),
        }
    } else if path.is_file() {
        print_if_err(f(path.to_path_buf()), path);
    } else {
        print_err("Invalid path given. Ensure last argument is a valid file or directory");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        print_err("Passed too few arguments. Run \"scfmt help\" for a list of valid options");
    } else if args.len() == 2 {
        let dir = &args[1];

        if dir == "help" {
            println!("{}", HELP_STR);
        } else if &args[1] == "version" || &args[1] == "v" {
            println!("scfmt {}", version!());
        } else {
            attempt_transform_path(scfmt::format_file, dir);
        }
    } else if args.len() == 3 {
        let flag = &args[1];
        let dir = &args[2];

        if flag == "add_brackets" || flag == "ab" {
            attempt_transform_path(scfmt::add_brackets_file, dir);
        } else if flag == "remove_brackets" || flag == "rb" {
            attempt_transform_path(scfmt::remove_brackets_file, dir);
        } else if flag == "null" || flag == "n" {
            attempt_transform_path(scfmt::null_existing_brackets_file, dir);
        } else {
            print_err("Invalid option given. Run \"scfmt help\" for a list of valid options");
        }
    } else {
        print_err("Passed too many arguments.");
    }
}
