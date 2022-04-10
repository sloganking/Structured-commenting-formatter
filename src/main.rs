use std::{env, path::PathBuf, sync::mpsc};
use strfmt::strfmt;
use threadpool::ThreadPool;
extern crate num_cpus;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        panic!("strfmt was passed too few arguments");
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
    rb, remove_brackets         Removes brackets from any bracketed structured comments"
            )
        } else {
            let path = PathBuf::from(dir);

            if path.is_dir() {
                // format dirs

                let pool = ThreadPool::new(num_cpus::get());
                let (results_tx, results_rx) = mpsc::channel();

                let paths = strfmt::get_files_in_dir(dir, "");
                for file in paths {
                    let results_tx1 = results_tx.clone();
                    pool.execute(move || {
                        strfmt::format_file(file);
                        results_tx1.send(()).unwrap();
                    });
                }

                // drop the transmitter we didn't clone
                drop(results_tx);

                // wait for threads to finish before exiting
                for _received in results_rx {}
            } else if path.is_file() {
                strfmt::format_file(path);
            } else {
                panic!("arg must be a file, path, or command")
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
                panic!("second arg must be a path or file");
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
                panic!("second arg must be a path or file");
            }
        } else {
            panic!("Invalid flag passed as arg");
        }
    } else {
        panic!("strfmt was passed too many arguments");
    }
}
