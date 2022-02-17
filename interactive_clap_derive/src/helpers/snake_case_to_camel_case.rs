pub fn snake_case_to_camel_case(s: String) -> String {
    let s_vec: Vec<String> = s
        .to_lowercase()
        .split("_")
        .map(|s| s.replacen(&s[..1], &s[..1].to_ascii_uppercase(), 1))
        .collect();
    s_vec.join("")
}
