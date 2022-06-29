use macros::static_array_from_file;

static_array_from_file!(BASE_TYPES, "mods/src/base_types.txt");

#[cfg(test)]
mod tests {
    use super::BASE_TYPES;

    #[test]
    fn check_size() {
        assert_eq!(3584, BASE_TYPES.len());
    }
}