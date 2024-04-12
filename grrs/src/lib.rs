
pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    // Iterate over each line and print pattern matches
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line);
        }
    }
}
