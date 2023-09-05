mod buffer_sum;
mod str_cmp;
mod new_vec;
use colored::*;

fn main() {
    let mut buffer = buffer_sum::Buffer::new();
    buffer.push(2);
    buffer.push(10);
    buffer.push(15);
    buffer.push(7);
    buffer.push(3);
    let sum = buffer.sum();
    println!("{} The sum is {}","part 1:".yellow(), sum);
    println!("{}", "part 2:".yellow());
    let x = "abcde";
    let y = "fghij";
    println!("'{}'>'{}'?    {}", x, y, str_cmp::compare_string(x, y));
    let x = "abcde";
    let y = "abcdf";
    println!("'{}'>'{}'?    {}", x, y, str_cmp::compare_string(x, y));
    let x = "abcde";
    let y = "abc";
    println!("'{}'>'{}'?    {}", x, y, str_cmp::compare_string(x, y));
    let x = "abcde";
    let y = "abcdef";
    println!("'{}'>'{}'?    {}", x, y, str_cmp::compare_string(x, y));
    let input = vec!['a','b','c','d','e'];
    println!("{} {:?}", "part 3:".yellow(), new_vec::next_char_vec(input));
}
