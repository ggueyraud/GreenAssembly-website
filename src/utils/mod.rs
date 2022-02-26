pub mod ua;

// Test pre-commit 2
pub fn extract_filename(full_filename: &str) -> Option<String> {
    full_filename
        .split('.')
        .collect::<Vec<_>>()
        .get(0)
        .map(|first| (*first).to_string())
}

#[cfg(test)]
mod tests {
    use super::extract_filename;

    #[test]
    fn extract_basic() {
        let filename = String::from("LoremIpsum_74.jpg");
        assert_eq!(
            extract_filename(&filename),
            Some(String::from("LoremIpsum_74"))
        );
    }
}
