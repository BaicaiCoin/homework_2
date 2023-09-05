pub fn next_char_vec(input: Vec<char>) ->Vec<char> {
    let output: Vec<char> = input.iter().map(|&c| (c as u8+1) as char).collect();
    output
}