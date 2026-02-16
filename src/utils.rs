pub fn remove_stop_words(input: Vec<String>) -> Vec<String> {
    let stop_words = stop_words::get(stop_words::LANGUAGE::English);
    let mut output: Vec<String> = Vec::new();

    for token in input {
        if !stop_words.contains(&token.as_str()) {
            output.push(token);
        }
    }

    output
}

pub fn sanitize_string(input: &str) -> String {
    let mut output = String::new();
    for c in input.chars() {
        let filtered_char = match c {
            'a'..'z' | 'A'..'Z' | '0'..'9' => c,
            _ => '_',
        };
        output.push(filtered_char);
    }
    output
}

pub fn tokenize_string(input: &str) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    let mut push_string = String::new();

    let input_chars_iter = input.chars();

    for c in input_chars_iter {
        match c {
            'a'..'z' | 'A'..'Z' | '0'..'9' => {
                push_string.push(c);
            }
            _ => {
                if push_string.trim().len() != 0 {
                    output.push(push_string);
                }
                push_string = String::new();
            }
        }
    }
    if push_string.trim().len() != 0 {
        output.push(push_string)
    }

    output
}
