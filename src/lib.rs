// #[cfg(test)]
// mod tests {
//     use crate::strfmt;

//     // #[test]
//     // fn integration_test() {
//     //     let paths = strfmt::get_files_in_dir("./test/", "");

//     //     for file in paths {
//     //         strfmt::format_file(file);
//     //     }
//     // }

//     // #[test]
//     // fn leading_spaces() {

//     // }
// }

pub mod strfmt {

    use glob::glob;
    use std::collections::HashMap;
    use std::fs;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;

    fn add_whitespace(line: &str, tab_depth: u32, tab_spaces: u32) -> String {
        let mut value = String::from("");

        for _i in 0..tab_depth * tab_spaces {
            value.push(' ');
        }

        value + line
    }

    pub fn get_files_in_dir(path: &str, filetype: &str) -> Vec<PathBuf> {
        //> get list of all files and dirs in ./input/ using glob
            let mut paths = Vec::new();
    
            let search_params = String::from(path) + "**/*" + filetype;
    
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

    fn gen_compatable_file_table() -> HashMap<&'static str, &'static str> {
        let mut filetype_to_comment = HashMap::new();
        filetype_to_comment.insert("java", "//");
        filetype_to_comment.insert("lua", "--");
        filetype_to_comment.insert("rs", "//");
        filetype_to_comment
    }

    pub fn format_str(str: &str, filetype: &str) -> Option<String> {
        //> determine if file compatible
            let filetype_to_comment = gen_compatable_file_table();
            let comment_starter = match filetype_to_comment.get(filetype) {
                Some(x) => *x,
                None => return None,
            };
        //<

        let mut formatted_file = String::from("");

        let tab_spaces = 4;
        let mut current_tab_depth = 0;
        let mut bracket_stack = Vec::new();

        let lines = str.lines();

        for (i, line) in lines.enumerate() {
            //> chop off begining spaces
                let mut line_no_leading_spaces = "";
                let char_vec: Vec<char> = line.chars().collect();
                for (i, char) in char_vec.iter().enumerate() {
                    if *char as u32 > 32 {
                        line_no_leading_spaces = &line[i..];
                        break;
                    }
                }
    
            //<> remove comment notation if it exists
                let comment_starter_with_space = comment_starter.to_owned() + " ";
                let mut is_a_comment = false;
                if line_no_leading_spaces.starts_with(&comment_starter_with_space) {
                    is_a_comment = true;
                    line_no_leading_spaces = &line_no_leading_spaces[comment_starter.len() + 1..];
                } else if line_no_leading_spaces.starts_with(comment_starter) {
                    is_a_comment = true;
                    line_no_leading_spaces = &line_no_leading_spaces[comment_starter.len()..];
                }
    
            //<> apply whitespace depth
                let formatted_line;
    
                if is_a_comment & line_no_leading_spaces.starts_with('>') {
                    formatted_line = add_whitespace(line, current_tab_depth, tab_spaces);
                    current_tab_depth += 1;
                    bracket_stack.push(i + 1);
                } else if is_a_comment & line_no_leading_spaces.starts_with("<>") {
                    if current_tab_depth == 0 {
                        panic!("<> closed nothing at line: {}", i + 1)
                    }
                    current_tab_depth -= 1;
                    formatted_line = add_whitespace(line, current_tab_depth, tab_spaces);
                    current_tab_depth += 1;
                    bracket_stack.pop();
                    bracket_stack.push(i + 1);
                } else if is_a_comment & line_no_leading_spaces.starts_with('<') {
                    if current_tab_depth == 0 {
                        panic!("< closed nothing at line: {}", i + 1)
                    }
                    current_tab_depth -= 1;
                    formatted_line = add_whitespace(line, current_tab_depth, tab_spaces);
                    bracket_stack.pop();
                } else {
                    formatted_line = add_whitespace(line, current_tab_depth, tab_spaces);
                }
            //<

            formatted_file.push_str(&(formatted_line + "\n"));
        }

        // remove last \n
        formatted_file.pop();

        //> ensure formatting successful
            if current_tab_depth != 0 {
                panic!("unclosed comment at line: {}", bracket_stack.pop().unwrap());
            }
        //<

        Some(formatted_file)
    }

    pub fn format_file(file: PathBuf) {
        let extenstion = file.extension().unwrap().to_str().unwrap();
        let contents = fs::read_to_string(&file).expect("Something went wrong reading the file");

        let formatted = format_str(&contents, extenstion).unwrap();

        //> write file
            let mut output = File::create(file).unwrap();
            write!(output, "{}", formatted).expect("failed to write file");
        //<
    }

    pub fn convert_to_brackets_file(file: PathBuf) {
        let extenstion = file.extension().unwrap().to_str().unwrap();
        let contents = fs::read_to_string(&file).expect("Something went wrong reading the file");

        let converted = convert_to_brackets(&contents, extenstion).unwrap();

        //> write file
            let mut output = File::create(file).unwrap();
            write!(output, "{}", converted).expect("failed to write file");
        //<
    }

    struct CommentDetail {
        line: usize,
        depth: usize,
    }

    fn make_comment_open_bracket(str: &str, filetype: &str) -> Option<String> {
        //> determine if file compatible
            let filetype_to_comment = gen_compatable_file_table();
            let comment_starter = match filetype_to_comment.get(filetype) {
                Some(x) => *x,
                None => return None,
            };
    
        //<> chop off begining spaces
            let mut line_no_leading_spaces = "";
            let mut leading_spaces: Option<usize> = None;
            let char_vec: Vec<char> = str.chars().collect();
            for (i, char) in char_vec.iter().enumerate() {
                if *char as u32 > 32 {
                    line_no_leading_spaces = &str[i..];
                    leading_spaces = Some(i);
                    break;
                }
            }
    
        //<> remove comment notation if it exists
            let comment_starter_with_space = comment_starter.to_owned() + " ";
            let mut is_a_comment = false;
            if line_no_leading_spaces.starts_with(&comment_starter_with_space)
                || line_no_leading_spaces.starts_with(comment_starter)
            {
                is_a_comment = true;
            }
        //<

        if !is_a_comment {
            return None;
        }

        let first_half = &str[..leading_spaces.unwrap() + comment_starter.len()];
        let second_half = &str[leading_spaces.unwrap() + comment_starter.len()..];

        Some(String::from(first_half) + ">" + second_half)
    }

    fn new_comment_closed_bracket(depth: usize, filetype: &str) -> Option<String> {
        //> determine if file compatible
            let filetype_to_comment = gen_compatable_file_table();
            let comment_starter = match filetype_to_comment.get(filetype) {
                Some(x) => *x,
                None => return None,
            };
        //<

        let mut result = String::new();
        for _i in 0..depth {
            result.push(' ');
        }

        result.push_str(&(String::from(comment_starter) + "<"));
        Some(result)
    }

    fn end_the_last_structured_comments(
        lines_list: &mut Vec<String>,
        comment_tracker: &mut Vec<CommentDetail>,
        cur_line: &mut usize,
        x: usize,
        filetype: &str,
    ) {
        while !comment_tracker.is_empty() && x <= comment_tracker[comment_tracker.len() - 1].depth {
            let close_bracket_line = new_comment_closed_bracket(
                comment_tracker[comment_tracker.len() - 1].depth,
                filetype,
            )
            .unwrap();
            lines_list.push(close_bracket_line);
            *cur_line += 1;
            comment_tracker.pop();
        }
    }

    fn pass_a_new_comment_that_we_dont_know_if_its_structured(
        lines_list: &mut Vec<String>,
        comment_tracker: &mut Vec<CommentDetail>,
        cur_line: &mut usize,
        leading_spaces: Option<usize>,
        unsure_if_last_comment_was_structured: &mut bool,
        line: &str,
    ) {
        let comment = CommentDetail {
            line: *cur_line,
            depth: leading_spaces.unwrap(),
        };

        comment_tracker.push(comment);
        *unsure_if_last_comment_was_structured = true;

        lines_list.push(String::from(line));
        *cur_line += 1;
    }

    pub fn convert_to_brackets(str: &str, filetype: &str) -> Option<String> {
        //> determine if file compatible
            let filetype_to_comment = gen_compatable_file_table();
            let comment_starter = match filetype_to_comment.get(filetype) {
                Some(x) => *x,
                None => return None,
            };
        //<

        let mut comment_tracker: Vec<CommentDetail> = Vec::new();

        let mut lines_list: Vec<String> = Vec::new();
        let mut unsure_if_last_comment_was_structured = true;

        let mut cur_line: usize = 0;
        for line in str.lines() {
            //> chop off begining spaces
                let mut line_no_leading_spaces = "";
                let mut leading_spaces: Option<usize> = None;
                let char_vec: Vec<char> = line.chars().collect();
                for (i, char) in char_vec.iter().enumerate() {
                    if *char as u32 > 32 {
                        line_no_leading_spaces = &line[i..];
                        leading_spaces = Some(i);
                        break;
                    }
                }
    
            //<> remove comment notation if it exists
                let comment_starter_with_space = comment_starter.to_owned() + " ";
                let mut is_a_comment = false;
                if line_no_leading_spaces.starts_with(&comment_starter_with_space)
                    || line_no_leading_spaces.starts_with(comment_starter)
                {
                    is_a_comment = true;
                }
            //<

            match leading_spaces {
                Some(x) => {
                    if is_a_comment {
                        if !comment_tracker.is_empty() {
                            if unsure_if_last_comment_was_structured {
                                if x > comment_tracker[comment_tracker.len() - 1].depth {
                                    // last was structured
                                    //> add bracket to last comment
    
                                        let line_with_no_bracket = &lines_list
                                            [comment_tracker[comment_tracker.len() - 1].line];
    
                                        lines_list[comment_tracker[comment_tracker.len() - 1].line] =
                                            make_comment_open_bracket(line_with_no_bracket, filetype)
                                                .unwrap();
                                    //<

                                    pass_a_new_comment_that_we_dont_know_if_its_structured(
                                        &mut lines_list,
                                        &mut comment_tracker,
                                        &mut cur_line,
                                        leading_spaces,
                                        &mut unsure_if_last_comment_was_structured,
                                        line,
                                    );
                                } else {
                                    // last was not structured
                                    comment_tracker.pop();

                                    end_the_last_structured_comments(
                                        &mut lines_list,
                                        &mut comment_tracker,
                                        &mut cur_line,
                                        x,
                                        filetype,
                                    );

                                    pass_a_new_comment_that_we_dont_know_if_its_structured(
                                        &mut lines_list,
                                        &mut comment_tracker,
                                        &mut cur_line,
                                        leading_spaces,
                                        &mut unsure_if_last_comment_was_structured,
                                        line,
                                    );
                                }
                            } else if x > comment_tracker[comment_tracker.len() - 1].depth {
                                pass_a_new_comment_that_we_dont_know_if_its_structured(
                                    &mut lines_list,
                                    &mut comment_tracker,
                                    &mut cur_line,
                                    leading_spaces,
                                    &mut unsure_if_last_comment_was_structured,
                                    line,
                                );
                            } else {
                                end_the_last_structured_comments(
                                    &mut lines_list,
                                    &mut comment_tracker,
                                    &mut cur_line,
                                    x,
                                    filetype,
                                );

                                pass_a_new_comment_that_we_dont_know_if_its_structured(
                                    &mut lines_list,
                                    &mut comment_tracker,
                                    &mut cur_line,
                                    leading_spaces,
                                    &mut unsure_if_last_comment_was_structured,
                                    line,
                                );
                            }
                        } else {
                            //> pass a comment that we don't know if it's structured
                                let comment = CommentDetail {
                                    line: cur_line,
                                    depth: leading_spaces.unwrap(),
                                };
    
                                comment_tracker.push(comment);
                                unsure_if_last_comment_was_structured = true;
    
                                lines_list.push(String::from(line));
                                cur_line += 1;
                            //<
                        }
                    } else if !comment_tracker.is_empty() {
                        if unsure_if_last_comment_was_structured {
                            if x > comment_tracker[comment_tracker.len() - 1].depth {
                                // last was structured
                                //> add bracket to last comment
    
                                    let line_with_no_bracket =
                                        &lines_list[comment_tracker[comment_tracker.len() - 1].line];
    
                                    lines_list[comment_tracker[comment_tracker.len() - 1].line] =
                                        make_comment_open_bracket(line_with_no_bracket, filetype)
                                            .unwrap();
                                //<
                            } else {
                                // last was not structured
                                comment_tracker.pop();

                                end_the_last_structured_comments(
                                    &mut lines_list,
                                    &mut comment_tracker,
                                    &mut cur_line,
                                    x,
                                    filetype,
                                );
                            }
                            unsure_if_last_comment_was_structured = false;

                            lines_list.push(String::from(line));
                            cur_line += 1;
                        } else if x > comment_tracker[comment_tracker.len() - 1].depth {
                            lines_list.push(String::from(line));
                            cur_line += 1;
                        } else {
                            end_the_last_structured_comments(
                                &mut lines_list,
                                &mut comment_tracker,
                                &mut cur_line,
                                x,
                                filetype,
                            );

                            //> forward the current line
                                lines_list.push(String::from(line));
                                cur_line += 1;
                            //<
                        }
                    } else {
                        lines_list.push(String::from(line));
                        cur_line += 1;
                    }
                }
                None => {
                    lines_list.push(String::from(line));
                    cur_line += 1;
                }
            }
        }

        let mut final_string = String::new();
        for line in lines_list {
            final_string.push_str(&line);
            final_string.push('\n');
        }
        // remove last \n
        final_string.pop();

        Some(final_string)
    }
}