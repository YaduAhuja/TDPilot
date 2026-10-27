pub fn camel_to_kebab(input: &str) -> String {
    let mut result = String::with_capacity(input.len());

    for ch in input.chars() {
        if ch.is_uppercase() {
            result.push('-');
            result.extend(ch.to_lowercase());
        } else {
            result.push(ch);
        }
    }

    result
}