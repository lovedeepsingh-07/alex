#[macro_export]
macro_rules! string_vec {
    ($($input_str:expr),*) => {
        {
            let mut output_vec = Vec::new();
            $(
                output_vec.push($input_str.to_string());
            )*
            output_vec
        }
    }
}

pub fn sanitize_string(input: &str) -> String {
    input
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect()
}

pub fn tokenize_string(input: &str) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    let mut push_string = String::new();

    let input_chars_iter = input.chars();

    for c in input_chars_iter {
        if c.is_ascii_alphanumeric() {
            push_string.push(c);
        } else {
            if !push_string.trim().is_empty() {
                output.push(push_string);
            }
            push_string = String::new();
        }
    }
    if !push_string.trim().is_empty() {
        output.push(push_string)
    }

    output
}
