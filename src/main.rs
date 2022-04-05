use strfmt::strfmt;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        //> format ./src/
            let paths = strfmt::get_files_in_dir("./src/", "");
    
            for file in paths {
                strfmt::format_file(file);
            }
        //<
    } else if args.len() == 2 {
        let flag = &args[1];

        if flag == "add_brackets" {
            //> convert to brackets
                let paths = strfmt::get_files_in_dir("./src/", "");
                for file in paths {
                    strfmt::convert_to_brackets_file(file);
                }
            //<
        } else if flag == "remove_brackets" {
            //> convert to bracketless
                let paths = strfmt::get_files_in_dir("./src/", "");
                for file in paths {
                    strfmt::convert_to_bracketless_file(file)
                }
            //<
        } else {
            panic!("Invalid flag passed as arg");
        }
    } else {
        panic!("strfmt was passed too many arguments");
    }
}