use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn add_whitespace(line: &str, tab_depth: u32, tab_spaces: u32) -> String{

    let mut value = String::from("");

    for _i in 0..tab_depth*tab_spaces{
        value.push(' ');
    }

    value + line
}

fn main() {
    // println!("Hello, world!");

    let mut formatted_file = String::from("");

    let tab_spaces = 4;
    let mut current_tab_depth = 0;

    

    if let Ok(lines) = read_lines("./input/formatted_lib.rs") {

        for line in lines {

            let line = line.expect("Line not valid");
            let mut starting_chars = String::from("");

            let char_vec: Vec<char> = line.chars().collect();
            for (i, char) in char_vec.iter().enumerate(){
                if *char as u32 > 32{
                    starting_chars = String::from(&line[i..]);
                    break;
                }
            }

            let formatted_line;

            // println!("starting_chars: {}", starting_chars);
            // println!("current_tab_depth: {}",current_tab_depth);

            if starting_chars.starts_with("//>"){
                formatted_line = add_whitespace(&line, current_tab_depth, tab_spaces);
                current_tab_depth += 1;
            } else if starting_chars.starts_with("//<>"){
                current_tab_depth -= 1;
                formatted_line = add_whitespace(&line, current_tab_depth, tab_spaces);
                current_tab_depth += 1;
            } else if starting_chars.starts_with("//<"){
                current_tab_depth -= 1;
                formatted_line = add_whitespace(&line, current_tab_depth, tab_spaces);
            }else{
                formatted_line = add_whitespace(&line, current_tab_depth, tab_spaces);
            }

            // println!("{}",formatted_line);

            formatted_file.push_str(&(formatted_line + "\n"));
        }
    }

    // remove last \n
    formatted_file.pop();

    println!("{}",formatted_file);

    // use std::fs::File;
    use std::io::{Write};

    let path = "./input/results.rs";
    let mut output = File::create(path).unwrap();
    write!(output, "{}", formatted_file).expect("failed to write file");

    // println!("{}",add_whitespace("Hello World!", 5, 4));
}