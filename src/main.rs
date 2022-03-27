use strfmt::strfmt;

fn main() {
    let paths = strfmt::get_files_in_dir("./src/", "rs");

    for file in paths {
        strfmt::format_file(file);
    }
}