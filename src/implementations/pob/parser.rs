use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::{
        complete::{alpha1, multispace0, newline, not_line_ending},
        is_alphabetic, is_newline,
    },
    sequence::{delimited, preceded, terminated},
    IResult,
};

fn rarity(i: &str) -> IResult<&str, &str> {
    preceded(tag("Rarity: "), alpha1)(i)
}

fn name(i: &str) -> IResult<&str, &str> {
    not_line_ending(i)
}

fn base_type(i: &str) -> IResult<&str, &str> {
    not_line_ending(i)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rarity_test() {
        assert_eq!(rarity(&"Rarity: RARE"), Ok((&""[..], &"RARE"[..])));
    }

    #[test]
    fn name_base_line_test() {
        assert_eq!(name(&"Loath Cut"), Ok((&""[..], &"Loath Cut"[..])));
        assert_eq!(
            base_type(&"Small Cluster Jewel"),
            Ok((&""[..], &"Small Cluster Jewel"[..]))
        );
    }
}
