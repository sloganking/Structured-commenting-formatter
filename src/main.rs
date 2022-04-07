use strfmt::strfmt;

use std::{env, path::PathBuf};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        panic!("strfmt was passed too few arguments");
    } else if args.len() == 2 {
        let dir = &args[1];
        let path = PathBuf::from(dir);

        if path.is_dir() {
            let paths = strfmt::get_files_in_dir(dir, "");
            for file in paths {
                strfmt::format_file(file);
            }
        } else if path.is_file() {
            strfmt::format_file(path);
        } else {
            panic!("arg must be a file, path, or command")
        }
    } else if args.len() == 3 {
        let flag = &args[1];

        if flag == "add_brackets" {
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
                panic!("second arg must be a path or file");
            }
        } else if flag == "remove_brackets" {
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
                panic!("second arg must be a path or file");
            }
        } else {
            panic!("Invalid flag passed as arg");
        }
    } else {
        panic!("strfmt was passed too many arguments");
    }
}
