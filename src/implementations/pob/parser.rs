use nom::{
    bytes::complete::{tag, take_while},
    character::complete::{alpha1, alphanumeric1, digit1, multispace0, newline, not_line_ending},
    combinator::{map_res, opt},
    error::ParseError,
    multi::{count, many0},
    sequence::{delimited, pair, preceded},
    IResult, Parser,
};
use std::str::FromStr;

fn cut_tag<'a, E: ParseError<&'a str>, F>(
    t: &'a str,
    ps: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str, E>
where
    F: Parser<&'a str, &'a str, E>,
{
    delimited(pair(opt(multispace0), tag(t)), ps, opt(newline))
}

fn rarity(i: &str) -> IResult<&str, &str> {
    cut_tag("Rarity: ", alpha1)(i)
}

fn name(i: &str) -> IResult<&str, &str> {
    preceded(opt(multispace0), not_line_ending)(i)
}

fn base_type(i: &str) -> IResult<&str, &str> {
    preceded(opt(multispace0), not_line_ending)(i)
}

fn unique_id(i: &str) -> IResult<&str, &str> {
    cut_tag("Unique ID: ", alphanumeric1)(i)
}

fn item_lvl(i: &str) -> IResult<&str, i32> {
    map_res(cut_tag("Item Level: ", digit1), |out: &str| {
        i32::from_str(out)
    })(i)
}

fn level_req(i: &str) -> IResult<&str, i32> {
    map_res(cut_tag("LevelReq: ", digit1), |out: &str| {
        i32::from_str(out)
    })(i)
}

fn implicits_count(i: &str) -> IResult<&str, i32> {
    map_res(cut_tag("Implicits: ", digit1), |out: &str| {
        i32::from_str(out)
    })(i)
}

fn affix(i: &str) -> IResult<&str, &str> {
    delimited(
        pair(opt(multispace0), opt(tag("{crafted}"))),
        take_while(|e: char| {
            e.is_alphabetic() || e.is_numeric() || e.is_ascii_punctuation() || e != '\n'
        }),
        opt(newline),
    )(i)
}

#[derive(Debug, Clone)]
pub struct PobItem {
    pub rarity: String,
    pub name: String,
    pub base_line: String,
    pub unique_id: String,
    pub item_lvl: i32,
    pub lvl_req: i32,
    pub implicits: Vec<String>,
    pub affixes: Vec<String>,
}

pub fn parse_pob_item(value: String) -> IResult<(), PobItem, ()> {
    let (input, rarity) = rarity(value.as_str())?;
    let (input, name) = name(input)?;
    let (input, base_line) = base_type(input)?;
    let (input, unique_id) = unique_id(input)?;
    let (input, item_lvl) = item_lvl(input)?;
    let (input, lvl_req) = level_req(input)?;
    let (input, impl_count) = implicits_count(input)?;
    let (mut input, implicits) = count(affix, impl_count as usize)(input)?;
    let mut affixes = vec![];
    while let Ok((i, affix)) = affix(input) {
        if !affix.is_empty() {
            affixes.push(affix);
        }

        if i.trim().is_empty() {
            break;
        }

        input = i;
    }

    Ok((
        (),
        PobItem {
            rarity: rarity.to_owned(),
            name: name.to_owned(),
            base_line: base_line.to_owned(),
            unique_id: unique_id.to_owned(),
            item_lvl,
            lvl_req,
            implicits: implicits.into_iter().map(|e| e.to_owned()).collect(),
            affixes: affixes.into_iter().map(|e| e.to_owned()).collect(),
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn whitespace_test() {
        assert!('\n'.is_whitespace());
    }

    #[test]
    fn rarity_test() {
        assert_eq!(rarity(&"Rarity: RARE"), Ok((&""[..], &"RARE"[..])));
        assert_eq!(rarity(&"\n\t\tRarity: RARE"), Ok((&""[..], &"RARE"[..])));
        assert_eq!(rarity(&"Rarity: RARE\n"), Ok((&""[..], &"RARE"[..])));
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
            affix(&"Added Small Passive Skills also grant: +3% to Chaos Resistance\n"),
            Ok((
                &""[..],
                &"Added Small Passive Skills also grant: +3% to Chaos Resistance"[..]
            ))
        );

        assert_eq!(affix(&""), Ok((&""[..], &""[..])));
    }

    #[test]
    fn simple_pob_item() -> Result<(), anyhow::Error> {
        let item = r#"Rarity: RARE
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
1 Added Passive Skill is Elegant Form"#;

        let (_, item) = parse_pob_item(&item)?;
        println!("{:?}", item);
        assert_eq!(item.rarity, "RARE");
        assert_eq!(item.name, "Loath Cut");
        assert_eq!(item.base_line, "Small Cluster Jewel");
        assert_eq!(
            item.unique_id,
            "c9ec1ff43acb2852474f462ce952d771edbf874f9710575a9e9ebd80b6e6dbfb"
        );
        assert_eq!(item.item_lvl, 84);
        assert_eq!(item.lvl_req, 54);
        assert_eq!(item.implicits.len(), 2);
        assert_eq!(item.affixes.len(), 4);
        Ok(())
    }
}
