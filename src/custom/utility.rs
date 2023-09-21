pub fn ordinal(n: u32) -> String {
    match n {
        1 | 21 | 31 => format!("{}st", n),
        2 | 22 => format!("{}nd", n),
        3 | 23 => format!("{}rd", n),
        _ => format!("{}th", n),
    }
}
