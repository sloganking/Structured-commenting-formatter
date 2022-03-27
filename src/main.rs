use strfmt::strfmt;

fn main() {
    //> format ./src/
        let paths = strfmt::get_files_in_dir("./src/", "rs");
    
        for file in paths {
            strfmt::format_file(file);
        }
    //<

    // //> format ./test/
    //     let paths = strfmt::get_files_in_dir("./test/", "lua");
    
    //     for file in paths {
    //         strfmt::format_file(file);
    //     }
    // //<

    //> convert to brackets
        // let paths = strfmt::get_files_in_dir("./test/", "lua");
        // for file in paths {
        //     strfmt::convert_to_brackets_file(file)
        // }
    //<

}