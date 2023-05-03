use macros::static_array_from_file;

pub static BASE_TYPES: &[&'static str] = static_array_from_file!("domain/src/base_types.txt");

#[cfg(test)]
mod tests {
    use super::BASE_TYPES;

    #[test]
    fn check_size() {
        assert_eq!(3584, BASE_TYPES.len());
    }
}