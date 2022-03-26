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
    let mut bracket_stack = Vec::new();

    if let Ok(lines) = read_lines(&file) {
        for (i, line) in lines.enumerate() {
            let line = line.expect("Line not valid");
            let mut line_no_leading_spaces = String::from("");

            //> chop off begining spaces
                let char_vec: Vec<char> = line.chars().collect();
                for (i, char) in char_vec.iter().enumerate() {
                    if *char as u32 > 32 {
                        line_no_leading_spaces = String::from(&line[i..]);
                        break;
                    }
                }
    
            //<> remove comment notation if it exists
                let comment_starter = "//";
                let mut is_a_comment = false;
                if line_no_leading_spaces.starts_with(comment_starter) {
                    is_a_comment = true;
                    line_no_leading_spaces =
                        String::from(&line_no_leading_spaces[comment_starter.len()..]);
                }
    
                // println!("is_a_comment: {}",is_a_comment);
                if is_a_comment {
                    println!();
                }
                println!("{}", line_no_leading_spaces);
    
            //<> apply whitespace depth
                let formatted_line;
    
                if is_a_comment & line_no_leading_spaces.starts_with(">") {
                    formatted_line = add_whitespace(&line, current_tab_depth, tab_spaces);
                    current_tab_depth += 1;
                    bracket_stack.push(i + 1);
                } else if is_a_comment & line_no_leading_spaces.starts_with("<>") {
                    current_tab_depth -= 1;
                    formatted_line = add_whitespace(&line, current_tab_depth, tab_spaces);
                    current_tab_depth += 1;
                    bracket_stack.pop();
                    bracket_stack.push(i + 1);
                } else if is_a_comment & line_no_leading_spaces.starts_with("<") {
                    current_tab_depth -= 1;
                    formatted_line = add_whitespace(&line, current_tab_depth, tab_spaces);
                    bracket_stack.pop();
                } else {
                    formatted_line = add_whitespace(&line, current_tab_depth, tab_spaces);
                }
            //<

            formatted_file.push_str(&(formatted_line + "\n"));
        }
    }

    // remove last \n
    formatted_file.pop();

    println!("{}", formatted_file);

    //> ensure formatting successful
        if current_tab_depth != 0 {
            panic!("unclosed comment at line: {}", bracket_stack.pop().unwrap());
        }
    
    //<> write file
        // let path = "./input/results.rs";
        let mut output = File::create(file).unwrap();
        write!(output, "{}", formatted_file).expect("failed to write file");
    //<
}

fn main() {
    let paths = get_rust_files_in_dir("./src/");

    for file in paths {
        format_file(file);
    }
}