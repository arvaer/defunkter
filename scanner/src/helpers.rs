pub fn is_digit(c: char) -> bool {
    return c >= '0' && c <= '9';
}

pub fn is_alpha(c: char) -> bool {
    return c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c == '_';
}

pub fn is_alphanumeric(c: char) -> bool {
    return is_alpha(c) || is_digit(c);
}
