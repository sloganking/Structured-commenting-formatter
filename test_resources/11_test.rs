/*
 *
 * Hello World!
 * 
 */

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
