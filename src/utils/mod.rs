pub mod ua;

pub fn
extract_filename(full_filename: &str) -> Option<String> {
    full_filename
        .split('.')
        .collect::<Vec<_>>()
        .get(0)
        .map(|first| (*first).to_string())
}
