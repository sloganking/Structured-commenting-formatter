#[cfg(test)]
mod tests {
    use std::fs;

    use crate::strfmt;

    #[test]
    fn format_str() {
        let to_format = fs::read_to_string("./test_resources/1_test.rs").unwrap();
        let answer = fs::read_to_string("./test_resources/1_answer.rs").unwrap();

        let formatted = strfmt::format_str(&to_format, "rs").unwrap();

        assert_eq!(answer, formatted);
    }
}

pub mod strfmt {

    use glob::glob;
    use std::collections::HashMap;
    use std::fs;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;

    // fn determine_whitespace_type(str: &str) -> (char,u32) {

    //     let mut chr = ' ';
    //     let mut num = 4;

    //     for line in str.lines(){
    //         match line.chars().nth(0){
    //             Some(x) => match x {
    //                 ' ' => {
    //                     num = 1;
    //                     for
    //                 },
    //                 '\t' => ,
    //                 _ => ,
    //             },
    //             None => continue,
    //         }
    //     }

    //     ('a',0)
    // }

    fn add_whitespace(line: &str, tab_depth: u32, tab_spaces: u32) -> String {
        let mut value = String::from("");

        for _i in 0..tab_depth * tab_spaces {
            value.push(' ');
        }

        value + line
    }

    fn set_whitespace(str: &str, depth: usize) -> String {
        let str_no_whitespace = match count_and_remove_begining_spaces(str) {
            Some(x) => x.1,
            None => "".to_owned(),
        };

        //> generate whitespace
            let mut whitespace = String::from("");
            for _i in 0..depth {
                whitespace.push(' ');
            }
        //<

        whitespace + &str_no_whitespace
    }

    pub fn get_files_in_dir(path: &str, filetype: &str) -> Vec<PathBuf> {
        //> get list of all files and dirs in ./input/ using glob
            let mut paths = Vec::new();
    
            let mut potential_slash = "";
            if PathBuf::from(path).is_dir() && !path.ends_with('/') {
                potential_slash = "/";
            }
    
            let search_params = String::from(path) + potential_slash + "**/*" + filetype;
    
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
        filetype_to_comment.insert("asm", ";");
        filetype_to_comment.insert("c", "//");
        filetype_to_comment.insert("cpp", "//");
        filetype_to_comment.insert("go", "//");
        //> Haskell
            filetype_to_comment.insert("hs", "--");
            filetype_to_comment.insert("lhs", "--");
        //<
        filetype_to_comment.insert("java", "//");
        //> JavaScript
            filetype_to_comment.insert("js", "//");
            filetype_to_comment.insert("cjs", "//");
            filetype_to_comment.insert("mjs", "//");
        //<
        filetype_to_comment.insert("lua", "--");
        filetype_to_comment.insert("rs", "//");
        filetype_to_comment.insert("sh", "#");
        filetype_to_comment.insert("sol", "//");
        //> TypeScript
            filetype_to_comment.insert("ts", "//");
            filetype_to_comment.insert("tsx", "//");
        //<
        filetype_to_comment
    }

    fn ensure_previous_lines_have_correct_whitespace(
        formatted_lines: &mut Vec<String>,
        comment_tracker: &mut Vec<CommentDetail>,
        tab_spaces: i32,
    ) {
        //> determine how much whitespace should be added
            let mut lowest_depth =
                comment_tracker[comment_tracker.len() - 1].depth + tab_spaces as usize;
            let line_of_last_unclosed_comment = comment_tracker[comment_tracker.len() - 1].line;
            for i in line_of_last_unclosed_comment + 1..formatted_lines.len() {
                let spaces_option = count_and_remove_begining_spaces(&formatted_lines[i]);
                match spaces_option {
                    Some(spaces_tuple) => {
                        if spaces_tuple.0 < lowest_depth {
                            lowest_depth = spaces_tuple.0;
                        }
                    }
                    None => continue,
                }
            }
        //<> add any needed whitespace
            if lowest_depth < comment_tracker[comment_tracker.len() - 1].depth + tab_spaces as usize {
                let depth_difference = comment_tracker[comment_tracker.len() - 1].depth
                    + tab_spaces as usize
                    - lowest_depth;
                if depth_difference > 0 {
                    for i in line_of_last_unclosed_comment + 1..formatted_lines.len() {
                        formatted_lines[i] =
                            add_whitespace(&formatted_lines[i], depth_difference.try_into().unwrap(), 1)
                    }
                }
            }
        //<
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
        let mut formatted_lines: Vec<String> = Vec::new();

        let tab_spaces = 4;

        let mut comment_tracker: Vec<CommentDetail> = Vec::new();

        let lines = str.lines();

        for (i, line) in lines.enumerate() {
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
                    formatted_line = line.to_string();
    
                    //> add comment to comment tracker
                        let comment = CommentDetail {
                            line: i,
                            depth: leading_spaces.unwrap(),
                        };
                        comment_tracker.push(comment);
                    //<
                } else if is_a_comment & line_no_leading_spaces.starts_with("<>") {
                    if comment_tracker.len() == 0 {
                        panic!("<> closed nothing at line: {}", i + 1)
                    }
    
                    ensure_previous_lines_have_correct_whitespace(
                        &mut formatted_lines,
                        &mut comment_tracker,
                        tab_spaces,
                    );
    
                    formatted_line =
                        set_whitespace(line, comment_tracker[comment_tracker.len() - 1].depth);
    
                    //> remove and add comment to comment tracker
                        let comment = CommentDetail {
                            line: i,
                            depth: comment_tracker[comment_tracker.len() - 1].depth,
                        };
                        comment_tracker.pop();
                        comment_tracker.push(comment);
                    //<
                } else if is_a_comment & line_no_leading_spaces.starts_with('<') {
                    if comment_tracker.is_empty() {
                        panic!("< closed nothing at line: {}", i + 1)
                    }
    
                    ensure_previous_lines_have_correct_whitespace(
                        &mut formatted_lines,
                        &mut comment_tracker,
                        tab_spaces,
                    );
    
                    formatted_line =
                        set_whitespace(line, comment_tracker[comment_tracker.len() - 1].depth);
    
                    // remove comment from comment tracker
                    comment_tracker.pop();
                } else {
                    formatted_line = line.to_string();
                }
            //<
            formatted_lines.push(formatted_line + "\n");
        }

        //> turn all lines into one string
            for line in formatted_lines {
                formatted_file.push_str(&line);
            }
        //<

        // remove last \n
        formatted_file.pop();

        //> ensure formatting successful
            if !comment_tracker.is_empty() {
                panic!(
                    "unclosed comment at line: {}",
                    comment_tracker[comment_tracker.len() - 1].line + 1
                );
            }
        //<
        Some(formatted_file)
    }

    pub fn format_file(file: PathBuf) -> bool {
        let extenstion = match file.extension() {
            Some(x) => match x.to_str() {
                Some(x) => x,
                None => return false,
            },
            None => return false,
        };

        let contents = match fs::read_to_string(&file) {
            Ok(x) => x,
            Err(_) => return false,
        };

        let formatted = match format_str(&contents, extenstion) {
            Some(x) => x,
            None => return false,
        };

        //> write file
            let mut output = match File::create(file) {
                Ok(x) => x,
                Err(_) => return false,
            };
    
            match write!(output, "{}", formatted) {
                Ok(x) => x,
                Err(_) => return false,
            };
        //<

        true
    }

    pub fn convert_to_brackets_file(file: PathBuf) -> bool {
        let extenstion = match file.extension() {
            Some(x) => match x.to_str() {
                Some(x) => x,
                None => return false,
            },
            None => return false,
        };

        let contents = match fs::read_to_string(&file) {
            Ok(x) => x,
            Err(_) => return false,
        };

        let converted = match convert_to_brackets(&contents, extenstion) {
            Some(x) => x,
            None => return false,
        };

        //> write file
            let mut output = match File::create(file) {
                Ok(x) => x,
                Err(_) => return false,
            };
    
            match write!(output, "{}", converted) {
                Ok(x) => x,
                Err(_) => return false,
            };
        //<

        true
    }

    struct CommentDetail {
        line: usize,
        depth: usize,
    }

    fn make_comment_closed_and_open_bracket(str: &str, comment_starter: &str) -> Option<String> {
        //> chop off begining spaces
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

        Some(String::from(first_half) + "<>" + second_half)
    }

    fn make_comment_open_bracket(str: &str, comment_starter: &str) -> Option<String> {
        //> chop off begining spaces
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

    fn new_comment_closed_bracket(depth: usize, comment_starter: &str) -> Option<String> {
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
        leading_spaces: usize,
        comment_starter: &str,
    ) {
        while !comment_tracker.is_empty()
            && leading_spaces <= comment_tracker[comment_tracker.len() - 1].depth
        {
            //> remove above whitespace
                while !lines_list.is_empty() && line_is_only_whitepace(lines_list.last().unwrap()) {
                    lines_list.pop();
                    *cur_line -= 1;
                }
            //<
            let close_bracket_line = new_comment_closed_bracket(
                comment_tracker[comment_tracker.len() - 1].depth,
                comment_starter,
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

    fn count_and_remove_begining_spaces(line: &str) -> Option<(usize, String)> {
        //> chop off begining spaces
            let mut line_no_leading_spaces = String::from("");
            let mut leading_spaces: Option<usize> = None;
            let char_vec: Vec<char> = line.chars().collect();
            for (i, char) in char_vec.iter().enumerate() {
                if *char as u32 > 32 {
                    line_no_leading_spaces = (&line[i..]).to_owned();
                    leading_spaces = Some(i);
                    break;
                }
            }
        //<
        match leading_spaces {
            Some(x) => Some((x, line_no_leading_spaces)),
            None => None,
        }
    }

    fn add_open_bracket_to_last_comment(
        lines_list: &mut Vec<String>,
        comment_tracker: &mut Vec<CommentDetail>,
        comment_starter: &str,
    ) {
        let mut should_consume_closing_comment = false;

        //> consume any previous now unecessary //<
    
            let line_of_latest_comment = comment_tracker[comment_tracker.len() - 1].line;
    
            // if there even could be a //< comment behind the lastest comment
            if line_of_latest_comment > 0 {
                let line_before_open_bracket_comment = &lines_list[line_of_latest_comment - 1];
    
                //> chop off begining spaces
                    let mut line_no_leading_spaces = "";
                    let mut leading_spaces: Option<usize> = None;
                    let char_vec: Vec<char> = line_before_open_bracket_comment.chars().collect();
                    for (i, char) in char_vec.iter().enumerate() {
                        if *char as u32 > 32 {
                            line_no_leading_spaces = &line_before_open_bracket_comment[i..];
                            leading_spaces = Some(i);
                            break;
                        }
                    }
        
                //<> remove comment notation if it exists
                    let comment_starter_with_space = comment_starter.to_owned() + " ";
                    let mut is_a_comment = false;
                    let mut line_no_comment_opener = "";
                    if line_no_leading_spaces.starts_with(&comment_starter_with_space) {
                        is_a_comment = true;
                        line_no_comment_opener = &line_no_leading_spaces[comment_starter.len() + 1..];
                    } else if line_no_leading_spaces.starts_with(comment_starter) {
                        is_a_comment = true;
                        line_no_comment_opener = &line_no_leading_spaces[comment_starter.len()..];
                    }
                //<
                let latest_comment =
                    match count_and_remove_begining_spaces(&lines_list[line_of_latest_comment]) {
                        Some(x) => x,
                        None => (0, String::from("")),
                    };
    
                if is_a_comment
                    && line_no_comment_opener.starts_with('<')
                    && latest_comment.0 == leading_spaces.unwrap()
                {
                    should_consume_closing_comment = true;
                }
            }
        //<
        let line_with_no_bracket = lines_list[line_of_latest_comment].clone();

        if should_consume_closing_comment {
            // overwrite the //< with whitespace
            lines_list[line_of_latest_comment - 1] = String::from("");

            // append brackets to latest comment
            lines_list[line_of_latest_comment] =
                make_comment_closed_and_open_bracket(&line_with_no_bracket, comment_starter)
                    .unwrap();
        } else {
            // append bracket to latest comment
            lines_list[line_of_latest_comment] =
                make_comment_open_bracket(&line_with_no_bracket, comment_starter).unwrap();
        }
    }

    fn line_is_only_whitepace(str: &str) -> bool {
        for char in str.chars() {
            if char as u32 > 32 {
                return false;
            }
        }
        true
    }

    pub fn convert_to_brackets(str: &str, filetype: &str) -> Option<String> {
        //> determine if file compatible
            let filetype_to_comment = gen_compatable_file_table();
            let comment_starter = match filetype_to_comment.get(filetype) {
                Some(x) => *x,
                None => return None,
            };
        //<
        // remove existing brackets, so later part of this function doesn't add more on top of existing ones.
        let str = &convert_to_bracketless(str, filetype).unwrap();

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
    
            //<> determine if line is a comment
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

                                    add_open_bracket_to_last_comment(
                                        &mut lines_list,
                                        &mut comment_tracker,
                                        comment_starter,
                                    );

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
                                        comment_starter,
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
                                    comment_starter,
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
                            pass_a_new_comment_that_we_dont_know_if_its_structured(
                                &mut lines_list,
                                &mut comment_tracker,
                                &mut cur_line,
                                leading_spaces,
                                &mut unsure_if_last_comment_was_structured,
                                line,
                            );
                        }
                    } else if !comment_tracker.is_empty() {
                        if unsure_if_last_comment_was_structured {
                            if x > comment_tracker[comment_tracker.len() - 1].depth {
                                // last was structured

                                add_open_bracket_to_last_comment(
                                    &mut lines_list,
                                    &mut comment_tracker,
                                    comment_starter,
                                );
                            } else {
                                // last was not structured

                                comment_tracker.pop();

                                end_the_last_structured_comments(
                                    &mut lines_list,
                                    &mut comment_tracker,
                                    &mut cur_line,
                                    x,
                                    comment_starter,
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
                                comment_starter,
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

        //> last comment was not structured, if it was the last non empty line in the String
            if unsure_if_last_comment_was_structured && !comment_tracker.is_empty() {
                comment_tracker.pop();
            }
        //<
        end_the_last_structured_comments(
            &mut lines_list,
            &mut comment_tracker,
            &mut cur_line,
            0,
            comment_starter,
        );

        let mut final_string = String::new();
        for line in lines_list {
            final_string.push_str(&line);
            final_string.push('\n');
        }
        // remove last \n
        final_string.pop();

        Some(final_string)
    }

    pub fn convert_to_bracketless_file(file: PathBuf) {
        let extenstion = file.extension().unwrap().to_str().unwrap();
        let contents = fs::read_to_string(&file).expect("Something went wrong reading the file");

        let converted = convert_to_bracketless(&contents, extenstion).unwrap();

        //> write file
            let mut output = File::create(file).unwrap();
            write!(output, "{}", converted).expect("failed to write file");
        //<
    }

    pub fn convert_to_bracketless(str: &str, filetype: &str) -> Option<String> {
        //> determine if file compatible
            let filetype_to_comment = gen_compatable_file_table();
            let comment_starter = match filetype_to_comment.get(filetype) {
                Some(x) => *x,
                None => return None,
            };
        //<

        //format str before removing brackets, to ensure their information is not lost.
        let str = format_str(str, filetype)?;

        let mut formatted_str = String::new();

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
                let mut line_no_comment_starter = "";
                let comment_starter_with_space = comment_starter.to_owned() + " ";
                let mut is_a_comment = false;
                if line_no_leading_spaces.starts_with(&comment_starter_with_space) {
                    is_a_comment = true;
                    line_no_comment_starter = &line_no_leading_spaces[comment_starter.len() + 1..];
                } else if line_no_leading_spaces.starts_with(comment_starter) {
                    is_a_comment = true;
                    line_no_comment_starter = &line_no_leading_spaces[comment_starter.len()..];
                }
            //<
            if is_a_comment {
                if line_no_comment_starter.starts_with("<>") {
                    formatted_str.push_str(
                        &(add_whitespace(
                            &(comment_starter.to_owned() + &line_no_comment_starter[2..]),
                            leading_spaces.unwrap().try_into().unwrap(),
                            1,
                        ) + "\n"),
                    );
                } else if line_no_comment_starter.starts_with('>') {
                    formatted_str.push_str(
                        &(add_whitespace(
                            &(comment_starter.to_owned() + &line_no_comment_starter[1..]),
                            leading_spaces.unwrap().try_into().unwrap(),
                            1,
                        ) + "\n"),
                    );
                } else if line_no_comment_starter.starts_with('<') {
                    // remove line
                    continue;
                } else {
                    formatted_str.push_str(&(line.to_owned() + "\n"));
                }
            } else {
                formatted_str.push_str(&(line.to_owned() + "\n"));
            }
        }

        // remove last '\n'
        formatted_str.pop();

        Some(formatted_str)
    }
}