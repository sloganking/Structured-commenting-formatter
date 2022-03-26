use strfmt::strfmt;

fn main() {
    let paths = strfmt::get_rust_files_in_dir("./src/");

    for file in paths {
        strfmt::format_file(file);
    }
}