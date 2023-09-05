pub fn compare_string(x: &str, y: &str) -> bool {
    let mut x_iter = x.chars();
    let mut y_iter = y.chars();
    loop {
        match (x_iter.next(), y_iter.next()) {
            (None, None) => return false,
            (Some(_), None) => return true,
            (None, Some(_)) => return false,
            (Some(ch1), Some(ch2)) => if ch1 != ch2 {
                return ch1 > ch2;
            }
        }
    }
}