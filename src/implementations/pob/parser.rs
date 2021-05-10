use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while},
    character::{
        complete::{alpha1, alphanumeric1, digit1, multispace0, newline, not_line_ending},
        is_alphabetic, is_newline,
    },
    combinator::{map_res, opt},
    sequence::{delimited, preceded, terminated},
    Finish, IResult,
};
use std::str::FromStr;
use thiserror::Error;

fn rarity(i: &str) -> IResult<&str, &str> {
    preceded(tag("Rarity: "), alpha1)(i)
}

fn name(i: &str) -> IResult<&str, &str> {
    not_line_ending(i)
}

fn base_type(i: &str) -> IResult<&str, &str> {
    not_line_ending(i)
}

fn unique_id(i: &str) -> IResult<&str, &str> {
    preceded(tag("Unique ID: "), alphanumeric1)(i)
}

fn item_lvl(i: &str) -> IResult<&str, i32> {
    map_res(preceded(tag("Item Level: "), digit1), |out: &str| {
        i32::from_str(out)
    })(i)
}

fn level_req(i: &str) -> IResult<&str, i32> {
    map_res(preceded(tag("LevelReq: "), digit1), |out: &str| {
        i32::from_str(out)
    })(i)
}

fn implicits_count(i: &str) -> IResult<&str, i32> {
    map_res(preceded(tag("Implicits: "), digit1), |out: &str| {
        i32::from_str(out)
    })(i)
}

fn affix(i: &str) -> IResult<&str, &str> {
    preceded(opt(tag("{crafted}")), not_line_ending)(i)
}

#[derive(Debug)]
pub struct PobItem<'a> {
    rarity: &'a str,
    name: &'a str,
    base_line: &'a str,
    unique_id: &'a str,
    item_lvl: i32,
    lvl_req: i32,
    affixes: Vec<&'a str>,
}

fn parse_pob_item<'a>(input: &'a str) -> IResult<&'a str, PobItem<'a>> {
    let (input, rarity) = rarity(input)?;
    let (input, name) = name(input)?;
    let (input, base_line) = base_type(input)?;
    let (input, unique_id) = unique_id(input)?;
    let (input, item_lvl) = item_lvl(input)?;
    let (input, lvl_req) = level_req(input)?;
    let (mut input, impl_count) = implicits_count(input)?;
    let mut implicits = vec![];
    for i in 0..impl_count {
        let (input, implicit) = affix(input)?;
        implicits.push(implicit);
    }
    Ok((
        input,
        PobItem {
            rarity,
            name,
            base_line,
            unique_id,
            item_lvl,
            lvl_req,
            affixes: implicits,
        },
    ))
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

    #[test]
    fn implicits_count_test() {
        assert_eq!(implicits_count(&"Implicits: 2"), Ok((&""[..], 2)));
    }

    #[test]
    fn affix_test() {
        assert_eq!(
            affix(&"{crafted}Adds 2 Passive Skills"),
            Ok((&""[..], &"Adds 2 Passive Skills"[..]))
        );
        assert_eq!(
            affix(&"Added Small Passive Skills also grant: +3% to Chaos Resistance"),
            Ok((
                &""[..],
                &"Added Small Passive Skills also grant: +3% to Chaos Resistance"[..]
            ))
        );
    }

    #[test]
    fn simple_pob_item() -> Result<(), anyhow::Error> {
        let item = r#"
			Rarity: RARE
Loath Cut
Small Cluster Jewel
Unique ID: c9ec1ff43acb2852474f462ce952d771edbf874f9710575a9e9ebd80b6e6dbfb
Item Level: 84
LevelReq: 54
Implicits: 2
{crafted}Adds 2 Passive Skills
{crafted}Added Small Passive Skills grant: 1% chance to Dodge Attack Hits
Added Small Passive Skills also grant: +3% to Chaos Resistance
Added Small Passive Skills also grant: +5 to Maximum Energy Shield
Added Small Passive Skills also grant: +5 to Strength
1 Added Passive Skill is Elegant Form
        "#;

        let (_, item) = parse_pob_item(&item)?;
        println!("{:?}", item);
        Ok(())
    }
}
