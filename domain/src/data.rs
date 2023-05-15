use lazy_static::lazy_static;
use macros::static_array_from_file;
use std::collections::HashMap;

pub fn cut_numbers(val: &str) -> String {
    val.replace(|el: char| el == '{' || el == '}' || el.is_numeric(), "")
}

pub static BASE_TYPES: &[&'static str] = static_array_from_file!("domain/src/base_types.txt");
pub static STATS_SIMPLE: &[&'static str] = static_array_from_file!("domain/src/stats.txt");
lazy_static! {
    pub static ref STATS_CUTTED: HashMap<String, usize> = {
        STATS_SIMPLE
            .iter()
            .enumerate()
            .map(|(idx, e)| (cut_numbers(*e), idx))
            .collect::<HashMap<String, usize>>()
    };
}

#[cfg(test)]
mod tests {
    use super::BASE_TYPES;

    #[test]
    fn check_size() {
        assert_eq!(3584, BASE_TYPES.len());
    }
}
