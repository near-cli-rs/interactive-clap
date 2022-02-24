pub fn to_kebab_case(s: String) -> String {
    let mut snake = String::new();
    for (i, ch) in s.char_indices() {
        if i > 0 && ch.is_uppercase() {
            snake.push('-');
        }
        snake.push(ch.to_ascii_lowercase());
    }
    snake.as_str().replace("_", "-")
}
