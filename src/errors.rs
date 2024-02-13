fn report(line: isize, location: &str, message: &str) {
    eprintln!("[line {}] Error {}: {}", line, location, message);
}

pub fn error(line: isize, message: &str) {
    report(line, "", message);
}
