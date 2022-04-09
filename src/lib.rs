#[cfg(test)]
mod tests {
    use crate::strfmt;
    use std::fs;

    //> basic tests
        #[test]
        fn format_str() {
            let to_format = fs::read_to_string("./test_resources/1_test.rs").unwrap();
            let answer = fs::read_to_string("./test_resources/1_answer.rs").unwrap();
            let formatted = strfmt::format_str(&to_format, "rs").unwrap();
            assert_eq!(answer, formatted);
        }

        #[test]
        fn add_brackets() {
            let to_format = fs::read_to_string("./test_resources/2_test.rs").unwrap();
            let answer = fs::read_to_string("./test_resources/2_answer.rs").unwrap();
            let formatted = strfmt::add_brackets(&to_format, "rs").unwrap();
            assert_eq!(answer, formatted);
        }

        #[test]
        fn remove_brackets() {
            let to_format = fs::read_to_string("./test_resources/3_test.rs").unwrap();
            let answer = fs::read_to_string("./test_resources/3_answer.rs").unwrap();
            let formatted = strfmt::remove_brackets(&to_format, "rs").unwrap();
            assert_eq!(answer, formatted);
        }

        #[test]
        fn no_change_without_brackets() {
            let before_formatting = fs::read_to_string("./test_resources/4_test.rs").unwrap();
            let formatted = strfmt::format_str(&before_formatting, "rs").unwrap();
            assert_eq!(formatted, before_formatting);
        }

    //<> Brackets not closed properly
        #[test]
        fn no_head_for_closing() {
            let before_formatting = fs::read_to_string("./test_resources/5_test.rs").unwrap();
            let formatted = strfmt::format_str(&before_formatting, "rs");
            assert_eq!(formatted, Err((46, "< closed nothing".to_owned())));
        }

        #[test]
        fn no_head_for_middle() {
            let before_formatting = fs::read_to_string("./test_resources/6_test.rs").unwrap();
            let formatted = strfmt::format_str(&before_formatting, "rs");
            assert_eq!(formatted, Err((21, "<> closed nothing".to_owned())));
        }

        #[test]
        fn head_never_closed() {
            let before_formatting = fs::read_to_string("./test_resources/7_test.rs").unwrap();
            let formatted = strfmt::format_str(&before_formatting, "rs");
            assert_eq!(formatted, Err((1, "unclosed comment".to_owned())));
        }

    //<> operations leave one empty line at end of string
        // #[test]
        // fn format_leaves_last_line_empty() {
        //     let formatted = strfmt::format_str("//>\n//<", "rs").unwrap();
        //     assert_eq!(formatted, "//>\n//<\n");
        // }

        // #[test]
        // fn add_brackets_leaves_last_line_empty() {
        //     let formatted = strfmt::add_brackets("// Hello World!\n    let a = 0;", "rs").unwrap();
        //     assert_eq!(formatted, "//> Hello World!\n    let a = 0;\n//<\n");
        // }

        // #[test]
        // fn remove_brackets_leaves_last_line_empty() {
        //     let formatted = strfmt::remove_brackets("//>\n//<", "rs").unwrap();
        //     assert_eq!(formatted, "//\n");
        // }

    //<> ending empty lines are preserved
        #[test]
        fn format_leaves_last_line_empty() {
            //> empty input
                let formatted = strfmt::format_str("", "rs").unwrap();
                assert_eq!(formatted, "");
            //> 0 empty ending lines
                let formatted = strfmt::format_str("//>\n//<", "rs").unwrap();
                assert_eq!(formatted, "//>\n//<");
            //<> 1 empty ending lines
                let formatted = strfmt::format_str("//>\n//<\n", "rs").unwrap();
                assert_eq!(formatted, "//>\n//<\n");
            //<> 2 empty ending lines
                let formatted = strfmt::format_str("//>\n//<\n\n", "rs").unwrap();
                assert_eq!(formatted, "//>\n//<\n\n");
            //<> 2 empty ending lines with space on last line
                let formatted = strfmt::format_str("//>\n//<\n\n ", "rs").unwrap();
                assert_eq!(formatted, "//>\n//<\n\n");
            //<
        }

    //<> tabs
        #[test]
        fn format_str_tabs() {
            let to_format = fs::read_to_string("./test_resources/9_test.rs").unwrap();
            let answer = fs::read_to_string("./test_resources/9_answer.rs").unwrap();
            let formatted = strfmt::format_str(&to_format, "rs").unwrap();
            assert_eq!(answer, formatted);
        }
    //<> tab depth of 2
        #[test]
        fn format_str_tab_depth_of_2() {
            let to_format = fs::read_to_string("./test_resources/10_test.rs").unwrap();
            let answer = fs::read_to_string("./test_resources/10_answer.rs").unwrap();
            let formatted = strfmt::format_str(&to_format, "rs").unwrap();
            assert_eq!(answer, formatted);
        }
    //<
}

pub mod strfmt {

    use colored::*;
    use glob::glob;
    use phf::phf_map;
    use std::fs;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;

    static EXTENSION_TO_COMMENT_STARTER_MAP: phf::Map<&'static str, &'static str> = phf_map! {

        //> ada
            "adb" => "--",
            "ads" => "--",
        //<
        // Assembly
        "asm" => ";",
        // AL
        "al" => "//",
        "bib" => "%",
        "brs" => "'",
        // C
        "c" => "//",
        "cfc" => "//",
        // Clojure
        "clj" => ";",
        // Apex
        "cls" => "//",
        "cpp" => "//",
        //> C#
            "cs" => "//",
            "csx" => "//",
        //<
        "d" => "//",
        // Dart
        "dart" => "//",
        "do" => "*",
        "ex" => "#",
        "elm" => "--",
        "gd" => "#",
        "gen" => "\\",
        // Go
        "go" => "//",
        "graphql" => "#",
        "groovy" => "//",
        //> Haskell
            "hs" => "--",
            "lhs" => "--",
        //<
        // Java
        "java" => "//",
        //> JavaScript
            "js" => "//",
            "cjs" => "//",
            "mjs" => "//",
        //<
        "jsonc" => "//",
        "lisp" => ";;",
        "lua" => "--",
        // MATLAB
        "m" => "%",
        "nim" => "#",
        // Pascal
        "pas" => "//",
        // PHP
        "php" => "//",
        "pig" => "--",
        "plsql" => "--",
        "pp" => "//",
        "ps1" => "#",
        "pu" => "'",
        "q" => "--",
        "rkt" => ";",
        // Rust
        "rs" => "//",
        "sas" => "*",
        "sass" => "//",
        "scss" => "//",
        "shader" => "//",
        // Bash
        "sh" => "#",
        // Solidity
        "sol" => "//",
        "styl" => "//",
        "svelte" => "//",
        "tcl" => "#",
        "toml" => "#",
        //> TypeScript
            "ts" => "//",
            "tsx" => "//",
        //<
        "vala" => "//",
        "v" => "//",
        "vhdl" => "--",
        "vue" => "//",
        "yaml" => "#",
    };

    fn determine_whitespace_type(str: &str) -> (char, usize) {
        //> if no whitespace is found, assume format is 4 spaces
            let mut chr = ' ';
            let mut num = 4;
        //<

        for line in str.lines() {
            if let Some(first_char) = line.chars().next() {
                if first_char == ' ' {
                    if let Some(whitespace) = count_and_remove_begining_whitespace(line) {
                        chr = ' ';
                        num = whitespace.0;
                        break;
                    }
                } else if first_char == '\t' {
                    chr = '\t';
                    num = 1;
                    break;
                }
            }
        }

        (chr, num)
    }

    fn add_whitespace(line: &str, depth: usize, whitespace_char: char) -> String {
        let mut value = String::from("");

        for _i in 0..depth {
            value.push(whitespace_char);
        }

        value + line
    }

    fn set_whitespace(str: &str, depth: usize, whitespace_char: char) -> String {
        let str_no_whitespace = match count_and_remove_begining_whitespace(str) {
            Some(x) => x.1,
            None => "".to_owned(),
        };

        //> generate whitespace
            let mut whitespace = String::from("");
            for _i in 0..depth {
                whitespace.push(whitespace_char);
            }
        //<

        whitespace + &str_no_whitespace
    }

    pub fn get_files_in_dir(path: &str, filetype: &str) -> Vec<PathBuf> {
        //> get list of all files and dirs in path, using glob
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

    fn ensure_previous_lines_have_correct_whitespace(
        formatted_lines: &mut Vec<String>,
        comment_tracker: &mut Vec<CommentDetail>,
        tab_spaces: usize,
        whitespace_char: char,
    ) {
        //> determine how much whitespace should be added
            let mut lowest_depth = comment_tracker[comment_tracker.len() - 1].depth + tab_spaces;
            let line_of_last_unclosed_comment = comment_tracker[comment_tracker.len() - 1].line;
            for i in line_of_last_unclosed_comment + 1..formatted_lines.len() {
                let whitespaces_option = count_and_remove_begining_whitespace(&formatted_lines[i]);
                match whitespaces_option {
                    Some(spaces_tuple) => {
                        if spaces_tuple.0 < lowest_depth {
                            lowest_depth = spaces_tuple.0;
                        }
                    }
                    None => continue,
                }
            }
        //<> add any needed whitespace
            if lowest_depth < comment_tracker[comment_tracker.len() - 1].depth + tab_spaces {
                let depth_difference =
                    comment_tracker[comment_tracker.len() - 1].depth + tab_spaces - lowest_depth;
                if depth_difference > 0 {
                    for i in line_of_last_unclosed_comment + 1..formatted_lines.len() {
                        match count_and_remove_begining_whitespace(&formatted_lines[i]) {
                            Some(_) => {
                                formatted_lines[i] = add_whitespace(
                                    &formatted_lines[i],
                                    depth_difference,
                                    whitespace_char,
                                )
                            }
                            None => formatted_lines[i] = "\n".to_owned(),
                        }
                    }
                }
            }
        //<
    }

    fn chop_off_beginning_spaces(line: &str) -> (Option<usize>, &str) {
        let mut line_no_leading_spaces = "";
        let mut leading_spaces: Option<usize> = None;
        for (i, char) in line.chars().enumerate() {
            if char as u32 > 32 {
                line_no_leading_spaces = &line[i..];
                leading_spaces = Some(i);
                break;
            }
        }

        (leading_spaces, line_no_leading_spaces)
    }

    fn remove_comment_notation_if_it_exists(line: &str, comment_starter: &str) -> (bool, String) {
        let mut line_no_comment_starter = line;
        let comment_starter_with_space = comment_starter.to_owned() + " ";
        let mut is_a_comment = false;
        if line_no_comment_starter.starts_with(&comment_starter_with_space) {
            is_a_comment = true;
            line_no_comment_starter = &line_no_comment_starter[comment_starter.len() + 1..];
        } else if line_no_comment_starter.starts_with(comment_starter) {
            is_a_comment = true;
            line_no_comment_starter = &line_no_comment_starter[comment_starter.len()..];
        }

        (is_a_comment, line_no_comment_starter.to_owned())
    }

    pub fn format_str(str: &str, filetype: &str) -> Result<String, (usize, String)> {
        // determine if file compatible
        let comment_starter = match EXTENSION_TO_COMMENT_STARTER_MAP.get(filetype) {
            Some(x) => *x,
            None => return Err((0, "Incompatible file type".to_owned())),
        };

        let mut formatted_file = String::from("");
        let mut formatted_lines: Vec<String> = Vec::new();
        let (whitespace_char, tab_spaces) = determine_whitespace_type(str);
        let mut comment_tracker: Vec<CommentDetail> = Vec::new();

        for (i, line) in str.lines().enumerate() {
            // chop off begining spaces
            let (leading_spaces, line_no_leading_spaces) = chop_off_beginning_spaces(line);

            // remove comment notation if it exists
            let (is_a_comment, line_no_leading_spaces) =
                remove_comment_notation_if_it_exists(line_no_leading_spaces, comment_starter);

            //> apply whitespace depth
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
                    if comment_tracker.is_empty() {
                        return Err((i + 1, "<> closed nothing".to_owned()));
                    }

                    ensure_previous_lines_have_correct_whitespace(
                        &mut formatted_lines,
                        &mut comment_tracker,
                        tab_spaces,
                        whitespace_char,
                    );

                    formatted_line = set_whitespace(
                        line,
                        comment_tracker[comment_tracker.len() - 1].depth,
                        whitespace_char,
                    );

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
                        return Err((i + 1, "< closed nothing".to_owned()));
                    }

                    ensure_previous_lines_have_correct_whitespace(
                        &mut formatted_lines,
                        &mut comment_tracker,
                        tab_spaces,
                        whitespace_char,
                    );

                    formatted_line = set_whitespace(
                        line,
                        comment_tracker[comment_tracker.len() - 1].depth,
                        whitespace_char,
                    );

                    // remove comment from comment tracker
                    comment_tracker.pop();
                } else {
                    if leading_spaces != None {
                        formatted_line = line.to_string();
                    } else {
                        // all whitespace only lines are set to depth 0
                        formatted_line = "".to_string();
                    }
                }
            //<
            formatted_lines.push(formatted_line + "\n");
        }

        //> turn all lines into one string
            for line in formatted_lines {
                formatted_file.push_str(&line);
            }

        //<

        // if the last char of source str wasn't '\n', don't add the last '\n'
        //> This prevents adding an additional empty line to our output, that wasn't in our input
            if let Some(last_char) = str.chars().last() {
                if last_char != '\n' {
                    // remove the last '\n'
                    formatted_file.pop();
                }
            }

        //<> ensure formatting successful
            if !comment_tracker.is_empty() {
                let err_line = comment_tracker[comment_tracker.len() - 1].line + 1;
                return Err((err_line, "unclosed comment".to_owned()));
            }
        //<
        Ok(formatted_file)
    }

    fn display_err(err: (usize, String), file: PathBuf) {
        if err.1 != "Incompatible file type" {
            println!("{}: {}", "Error".red().bold(), err.1);
            println!(
                "{}",
                file.as_os_str().to_str().unwrap().to_owned() + ":" + &format!("{}", err.0)
            );
        }
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
            Ok(x) => x,
            Err(err) => {
                display_err(err, file);
                return false;
            }
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

    pub fn add_brackets_file(file: PathBuf) -> bool {
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

        let converted = match add_brackets(&contents, extenstion) {
            Ok(x) => x,
            Err(err) => {
                display_err(err, file);
                return false;
            }
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

    fn make_comment_closed_and_open_bracket(line: &str, comment_starter: &str) -> Option<String> {
        let (leading_spaces, line_no_leading_spaces) = chop_off_beginning_spaces(line);

        // remove comment notation if it exists
        let (is_a_comment, _line_no_comment_starter) =
            remove_comment_notation_if_it_exists(line_no_leading_spaces, comment_starter);

        if !is_a_comment {
            return None;
        }

        let first_half = &line[..leading_spaces.unwrap() + comment_starter.len()];
        let second_half = &line[leading_spaces.unwrap() + comment_starter.len()..];

        Some(String::from(first_half) + "<>" + second_half)
    }

    fn make_comment_open_bracket(line: &str, comment_starter: &str) -> Option<String> {
        // chop off begining spaces
        let (leading_spaces, line_no_leading_spaces) = chop_off_beginning_spaces(line);

        // remove comment notation if it exists
        let (is_a_comment, _line_no_comment_starter) =
            remove_comment_notation_if_it_exists(line_no_leading_spaces, comment_starter);

        if !is_a_comment {
            return None;
        }

        let first_half = &line[..leading_spaces.unwrap() + comment_starter.len()];
        let second_half = &line[leading_spaces.unwrap() + comment_starter.len()..];

        Some(String::from(first_half) + ">" + second_half)
    }

    fn new_comment_closed_bracket(
        depth: usize,
        comment_starter: &str,
        whitespace_char: char,
    ) -> Option<String> {
        let mut result = String::new();
        for _i in 0..depth {
            result.push(whitespace_char);
        }

        result.push_str(&(String::from(comment_starter) + "<"));
        Some(result)
    }

    fn remove_empty_tail(lines_list: &mut Vec<String>) {
        while !lines_list.is_empty() && line_is_only_whitepace(lines_list.last().unwrap()) {
            lines_list.pop();
        }
    }

    fn end_the_last_structured_comments(
        lines_list: &mut Vec<String>,
        comment_tracker: &mut Vec<CommentDetail>,
        leading_spaces: usize,
        comment_starter: &str,
        whitespace_char: char,
    ) {
        while !comment_tracker.is_empty()
            && leading_spaces <= comment_tracker[comment_tracker.len() - 1].depth
        {
            let empty_line_count = count_ending_empty_lines(lines_list);

            // remove above whitespace
            remove_empty_tail(lines_list);

            let close_bracket_line = new_comment_closed_bracket(
                comment_tracker[comment_tracker.len() - 1].depth,
                comment_starter,
                whitespace_char,
            )
            .unwrap();
            lines_list.push(close_bracket_line);
            comment_tracker.pop();

            append_num_empty_lines(empty_line_count, lines_list);
        }
    }

    fn pass_a_new_comment_that_we_dont_know_if_its_structured(
        lines_list: &mut Vec<String>,
        comment_tracker: &mut Vec<CommentDetail>,
        leading_spaces: Option<usize>,
        unsure_if_last_comment_was_structured: &mut bool,
        line: &str,
    ) {
        let comment = CommentDetail {
            line: lines_list.len(),
            depth: leading_spaces.unwrap(),
        };

        comment_tracker.push(comment);
        *unsure_if_last_comment_was_structured = true;

        lines_list.push(String::from(line));
    }

    fn count_and_remove_begining_whitespace(line: &str) -> Option<(usize, String)> {
        // chop off begining spaces
        let (leading_whitespace_option, line_no_leading_spaces) = chop_off_beginning_spaces(line);

        match leading_whitespace_option {
            Some(num_leading_whitespace) => {
                Some((num_leading_whitespace, line_no_leading_spaces.to_owned()))
            }
            None => None,
        }
    }

    fn last_non_empty_line_before_index(
        index: usize,
        lines_list: &Vec<String>,
    ) -> Option<(usize, &str)> {
        for i in (0..index).rev() {
            if !line_is_only_whitepace(&lines_list[i]) {
                return Some((i, &lines_list[i]));
            }
        }

        None
    }

    fn add_open_bracket_to_last_comment(
        lines_list: &mut Vec<String>,
        comment_tracker: &mut Vec<CommentDetail>,
        comment_starter: &str,
    ) {
        let mut should_consume_closing_comment = false;

        //> consume any previous now unecessary //<

            let line_of_latest_comment = comment_tracker[comment_tracker.len() - 1].line;

            let last_solid_line_option =
                last_non_empty_line_before_index(line_of_latest_comment, lines_list);

            // if there even could be a //< comment behind the lastest comment
            if let Some((_last_solid_line_index, line_before_open_bracket_comment)) =
                last_solid_line_option
            {
                // chop off begining spaces
                let (leading_spaces, line_no_leading_spaces) =
                    chop_off_beginning_spaces(line_before_open_bracket_comment);

                // remove comment notation if it exists
                let (is_a_comment, line_no_comment_opener) =
                    remove_comment_notation_if_it_exists(line_no_leading_spaces, comment_starter);

                let latest_comment =
                    match count_and_remove_begining_whitespace(&lines_list[line_of_latest_comment]) {
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
            //> pop everything to the last //<, but remember how to restore what was popped.
                let after_spaces = count_ending_empty_lines(lines_list);
                remove_empty_tail(lines_list);

                // remove the soon to be bracketed comment
                // we'll add it back later
                lines_list.pop();

                let before_spaces = count_ending_empty_lines(lines_list);
                remove_empty_tail(lines_list);

            //<> remove the //<
                lines_list.pop();

            //<> put things back and make add brackets to latest comment

                append_num_empty_lines(before_spaces, lines_list);

                // re-append the latest comment, with added brackets
                lines_list.push(
                    make_comment_closed_and_open_bracket(&line_with_no_bracket, comment_starter)
                        .unwrap(),
                );

                append_num_empty_lines(after_spaces, lines_list);
            //<
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

    pub fn add_brackets(str: &str, filetype: &str) -> Result<String, (usize, String)> {
        // determine if file compatible
        let comment_starter = match EXTENSION_TO_COMMENT_STARTER_MAP.get(filetype) {
            Some(x) => *x,
            None => return Err((0, "Incompatible file type".to_owned())),
        };

        // remove existing brackets, so later part of this function doesn't add more on top of existing ones.
        let str = &remove_brackets(str, filetype)?;

        let (whitespace_char, _tab_spaces) = determine_whitespace_type(str);

        let mut comment_tracker: Vec<CommentDetail> = Vec::new();

        let mut lines_list: Vec<String> = Vec::new();
        let mut unsure_if_last_comment_was_structured = true;

        for line in str.lines() {
            // chop off begining spaces
            let (leading_spaces, line_no_leading_spaces) = chop_off_beginning_spaces(line);

            let (is_a_comment, _) =
                remove_comment_notation_if_it_exists(line_no_leading_spaces, comment_starter);

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
                                        x,
                                        comment_starter,
                                        whitespace_char,
                                    );

                                    pass_a_new_comment_that_we_dont_know_if_its_structured(
                                        &mut lines_list,
                                        &mut comment_tracker,
                                        leading_spaces,
                                        &mut unsure_if_last_comment_was_structured,
                                        line,
                                    );
                                }
                            } else if x > comment_tracker[comment_tracker.len() - 1].depth {
                                pass_a_new_comment_that_we_dont_know_if_its_structured(
                                    &mut lines_list,
                                    &mut comment_tracker,
                                    leading_spaces,
                                    &mut unsure_if_last_comment_was_structured,
                                    line,
                                );
                            } else {
                                end_the_last_structured_comments(
                                    &mut lines_list,
                                    &mut comment_tracker,
                                    x,
                                    comment_starter,
                                    whitespace_char,
                                );

                                pass_a_new_comment_that_we_dont_know_if_its_structured(
                                    &mut lines_list,
                                    &mut comment_tracker,
                                    leading_spaces,
                                    &mut unsure_if_last_comment_was_structured,
                                    line,
                                );
                            }
                        } else {
                            pass_a_new_comment_that_we_dont_know_if_its_structured(
                                &mut lines_list,
                                &mut comment_tracker,
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
                                    x,
                                    comment_starter,
                                    whitespace_char,
                                );
                            }
                            unsure_if_last_comment_was_structured = false;

                            lines_list.push(String::from(line));
                        } else if x > comment_tracker[comment_tracker.len() - 1].depth {
                            lines_list.push(String::from(line));
                        } else {
                            end_the_last_structured_comments(
                                &mut lines_list,
                                &mut comment_tracker,
                                x,
                                comment_starter,
                                whitespace_char,
                            );

                            // forward the current line
                            lines_list.push(String::from(line));
                        }
                    } else {
                        lines_list.push(String::from(line));
                    }
                }
                None => {
                    lines_list.push(String::from("".to_owned()));
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
            0,
            comment_starter,
            whitespace_char,
        );

        remove_empty_tail(&mut lines_list);

        //> turn all lines into one string
            let mut final_string = String::new();
            for line in lines_list {
                final_string.push_str(&line);
                final_string.push('\n');
            }
        //<

        Ok(final_string)
    }

    pub fn remove_brackets_file(file: PathBuf) -> bool {
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

        let converted = match remove_brackets(&contents, extenstion) {
            Ok(x) => x,
            Err(err) => {
                display_err(err, file);
                return false;
            }
        };

        //> write file
            let mut output = File::create(file).unwrap();
            write!(output, "{}", converted).expect("failed to write file");
        //<

        true
    }

    fn line_is_a_comment(str: &str, comment_starter: &str) -> bool {
        match count_and_remove_begining_whitespace(str) {
            Some(x) => {
                let comment_starter_with_space = comment_starter.to_owned() + " ";
                let str = x.1;

                if str.starts_with(&comment_starter_with_space) {
                    true
                } else {
                    str.starts_with(comment_starter)
                }
            }
            None => false,
        }
    }

    fn remove_comment_starter(str: &str, comment_starter: &str) -> String {
        match count_and_remove_begining_whitespace(str) {
            Some(x) => {
                let str = x.1;

                let mut line_no_comment_starter = "";
                let comment_starter_with_space = comment_starter.to_owned() + " ";
                if str.starts_with(&comment_starter_with_space) {
                    line_no_comment_starter = &str[comment_starter.len() + 1..];
                } else if str.starts_with(comment_starter) {
                    line_no_comment_starter = &str[comment_starter.len()..];
                }

                line_no_comment_starter.to_string()
            }
            None => str.to_owned(),
        }
    }

    fn count_ending_empty_lines(lines_list: &Vec<String>) -> usize {
        let mut count = 0;
        for i in (0..lines_list.len()).rev() {
            if !line_is_only_whitepace(&lines_list[i]) {
                break;
            }
            count += 1;
        }

        count
    }

    fn append_num_empty_lines(num: usize, lines_list: &mut Vec<String>) {
        for _ in 0..num {
            lines_list.push("".to_owned());
        }
    }

    pub fn remove_brackets(str: &str, filetype: &str) -> Result<String, (usize, String)> {
        // determine if file compatible
        let comment_starter = match EXTENSION_TO_COMMENT_STARTER_MAP.get(filetype) {
            Some(x) => *x,
            None => return Err((0, "Incompatible file type".to_owned())),
        };

        let mut lines_list: Vec<String> = Vec::new();

        //format str before removing brackets, to ensure their information is not lost.
        let str = &format_str(str, filetype)?;

        let (whitespace_char, _tab_spaces) = determine_whitespace_type(str);

        let mut formatted_str = String::new();

        for line in str.lines() {
            let line_no_leading_whitespace;
            let leading_whitespace;

            if let Some(x) = count_and_remove_begining_whitespace(line) {
                leading_whitespace = x.0;
                line_no_leading_whitespace = &x.1;

                if line_is_a_comment(line_no_leading_whitespace, comment_starter) {
                    let line_no_comment_starter =
                        remove_comment_starter(line_no_leading_whitespace, comment_starter);

                    if line_no_comment_starter.starts_with("<>") {
                        lines_list.push(
                            add_whitespace(
                                &(comment_starter.to_owned() + &line_no_comment_starter[2..]),
                                leading_whitespace,
                                whitespace_char,
                            ) + "\n",
                        );
                    } else if line_no_comment_starter.starts_with('>') {
                        lines_list.push(
                            add_whitespace(
                                &(comment_starter.to_owned() + &line_no_comment_starter[1..]),
                                leading_whitespace,
                                whitespace_char,
                            ) + "\n",
                        );
                    } else if line_no_comment_starter.starts_with('<') {
                        // remove line by not adding it to output
                        continue;
                    } else {
                        lines_list.push(line.to_owned() + "\n");
                    }
                } else {
                    lines_list.push(line.to_owned() + "\n");
                }
            } else {
                lines_list.push("\n".to_owned());
            }
        }

        remove_empty_tail(&mut lines_list);

        //> turn all lines into one string
            for line in lines_list {
                formatted_str.push_str(&line);
            }
        //<

        Ok(formatted_str)
    }
}
