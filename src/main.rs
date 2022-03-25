use glob::glob;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn add_whitespace(line: &str, tab_depth: u32, tab_spaces: u32) -> String {
    let mut value = String::from("");

    for _i in 0..tab_depth * tab_spaces {
        value.push(' ');
    }

    value + line
}

fn get_rust_files_in_dir(path: &str) -> Vec<PathBuf> {
    //> get list of all files in ./input/ using glob
        let mut paths = Vec::new();
    
        let file_delimiter = "rs";
        let search_params = String::from(path) + "**/*" + file_delimiter;
    
        for entry in glob(&search_params).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    paths.push(path);
                }
                Err(e) => println!("{:?}", e),
            }
        }
    
    //<> filter out directories
        let paths = paths.into_iter().filter(|e| e.is_file());
    
    //<> filter out non unicode files
        let paths: Vec<PathBuf> = paths
            .into_iter()
            .filter(|e| fs::read_to_string(e).is_ok())
            .collect();
    //<

    paths
}

fn format_file(file: PathBuf) {
    let mut formatted_file = String::from("");

    let tab_spaces = 4;
    let mut current_tab_depth = 0;

    if let Ok(lines) = read_lines(&file) {
        for line in lines {
            let line = line.expect("Line not valid");
            let mut starting_chars = String::from("");

            let char_vec: Vec<char> = line.chars().collect();
            for (i, char) in char_vec.iter().enumerate() {
                if *char as u32 > 32 {
                    starting_chars = String::from(&line[i..]);
                    break;
                }
            }

            let formatted_line;

            if starting_chars.starts_with("//>") {
                formatted_line = add_whitespace(&line, current_tab_depth, tab_spaces);
                current_tab_depth += 1;
            } else if starting_chars.starts_with("//<>") {
                current_tab_depth -= 1;
                formatted_line = add_whitespace(&line, current_tab_depth, tab_spaces);
                current_tab_depth += 1;
            } else if starting_chars.starts_with("//<") {
                current_tab_depth -= 1;
                formatted_line = add_whitespace(&line, current_tab_depth, tab_spaces);
            } else {
                formatted_line = add_whitespace(&line, current_tab_depth, tab_spaces);
            }

            formatted_file.push_str(&(formatted_line + "\n"));
        }
    }

    // remove last \n
    formatted_file.pop();

    println!("{}", formatted_file);

    //> write file
        // let path = "./input/results.rs";
        let mut output = File::create(file).unwrap();
        write!(output, "{}", formatted_file).expect("failed to write file");
    //<

    assert!(current_tab_depth == 0, "unclosed comment");
}

fn main() {
    let paths = get_rust_files_in_dir("./src/");

    for file in paths {
        format_file(file);
    }
}