use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{alpha1, alphanumeric1, digit1, line_ending, not_line_ending},
    combinator::{cut, map, map_res, opt},
    error::{context, ContextError, FromExternalError, ParseError},
    multi::{many0, many_m_n},
    sequence::{preceded, terminated},
    IResult,
};
use std::num::ParseIntError;
use std::str::FromStr;
use tracing::info;

#[derive(Debug, PartialEq)]
enum ItemValue {
    Rarity(String),
    Name(String, Option<String>),
    ItemLevel(i32),
    LevelReq(i32),
    Implicits(Vec<String>),
    UniqueId(String),
    Affix(String),
    Quality(i32),
    Sockets(String),
}

static EMPTY_BASETYPES: &[&str] = &["Flask"];

fn sp<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = " \t\r\n ";

    // nom combinators like `take_while` return a function. That function is the
    // parser,to which we can pass the input
    take_while(move |c| chars.contains(c))(i)
}

fn rarity<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    context("rarity", preceded(tag("Rarity: "), cut(alpha1)))(i)
}

fn name<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, (&'a str, Option<&'a str>), E> {
    let (i, name) = context("name", cut(not_line_ending))(i)?;

    if !EMPTY_BASETYPES.iter().any(|el| name.contains(el)) {
        let (i, basetype) = base_type(i)?;
        return Ok((i, (name, Some(basetype))));
    }

    Ok((i, (name, None)))
}

fn base_type<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    context("base_type", preceded(sp, cut(not_line_ending)))(i)
}

fn unique_id<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    context(
        "unique_id",
        preceded(tag("Unique ID: "), cut(alphanumeric1)),
    )(i)
}

fn sockets<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    context("sockets", preceded(tag("Sockets: "), cut(not_line_ending)))(i)
}

fn quality<
    'a,
    E: ParseError<&'a str> + FromExternalError<&'a str, ParseIntError> + ContextError<&'a str>,
>(
    i: &'a str,
) -> IResult<&'a str, i32, E> {
    context(
        "quality",
        map_res(preceded(tag("Quality: "), cut(digit1)), |out: &str| {
            i32::from_str(out)
        }),
    )(i)
}

fn item_lvl<
    'a,
    E: ParseError<&'a str> + FromExternalError<&'a str, ParseIntError> + ContextError<&'a str>,
>(
    i: &'a str,
) -> IResult<&'a str, i32, E> {
    context(
        "item_lvl",
        map_res(preceded(tag("Item Level: "), cut(digit1)), |out: &str| {
            i32::from_str(out)
        }),
    )(i)
}

fn level_req<'a, E>(i: &'a str) -> IResult<&'a str, i32, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, ParseIntError> + ContextError<&'a str>,
{
    context(
        "level_req",
        map_res(preceded(tag("LevelReq: "), cut(digit1)), |out: &str| {
            i32::from_str(out)
        }),
    )(i)
}

fn implicits<'a, E>(i: &'a str) -> IResult<&'a str, Vec<&'a str>, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, ParseIntError> + ContextError<&'a str>,
{
    let (i, implicits_count) = context(
        "implicits_count",
        map_res(preceded(tag("Implicits: "), cut(digit1)), |out: &str| {
            i32::from_str(out)
        }),
    )(i)?;

    context(
        "implicits",
        many_m_n(
            implicits_count as usize,
            implicits_count as usize,
            preceded(sp, affix),
        ),
    )(i)
}

fn affix<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    context(
        "affix",
        preceded(opt(tag("{crafted}")), cut(not_line_ending)),
    )(i)
}

fn item_value_header<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, ItemValue, E> {
    context(
        "item_value_header",
        preceded(
            sp,
            alt((
                map(rarity, |s| ItemValue::Rarity(String::from(s))),
                map(name, |(n, b)| {
                    ItemValue::Name(String::from(n), b.map(String::from))
                }),
            )),
        ),
    )(i)
}

fn item_value<
    'a,
    E: ParseError<&'a str> + FromExternalError<&'a str, ParseIntError> + ContextError<&'a str>,
>(
    i: &'a str,
) -> IResult<&'a str, ItemValue, E> {
    context(
        "item_value",
        preceded(
            sp,
            alt((
                map(unique_id, |s| ItemValue::UniqueId(String::from(s))),
                map(item_lvl, ItemValue::ItemLevel),
                map(level_req, ItemValue::LevelReq),
                map(implicits, |vals| {
                    ItemValue::Implicits(vals.into_iter().map(String::from).collect())
                }),
                map(quality, ItemValue::Quality),
                map(sockets, |s| ItemValue::Sockets(String::from(s))),
                map(affix, |s| ItemValue::Affix(String::from(s))),
            )),
        ),
    )(i)
}

fn root<
    'a,
    E: ParseError<&'a str> + FromExternalError<&'a str, ParseIntError> + ContextError<&'a str>,
>(
    i: &'a str,
) -> IResult<&'a str, Vec<ItemValue>, E> {
    let (i, mut header_vals) = many_m_n(2, 2, terminated(item_value_header, line_ending))(i)?;
    let (i, mut vals) = many0(terminated(item_value, line_ending))(i)?;
    let (i, end_val) = item_value(i)?;
    header_vals.append(&mut vals);
    header_vals.push(end_val);
    Ok((i, header_vals))
}

#[derive(Debug, Clone, Default)]
pub struct ParsedItem {
    pub rarity: String,
    pub name: String,
    pub base_line: String,
    pub unique_id: String,
    pub item_lvl: i32,
    pub lvl_req: i32,
    pub sockets: String,
    pub quality: i32,
    pub implicits: Vec<String>,
    pub affixes: Vec<String>,
}

pub fn parse_pob_item<'a, E>(i: &'a str) -> IResult<&'a str, ParsedItem, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, ParseIntError> + ContextError<&'a str>,
{
    let (i, items) = root(i)?;
    let mut item = ParsedItem::default();

    for val in items {
        match val {
            ItemValue::Rarity(r) => item.rarity = r,
            ItemValue::Name(name, Some(base)) => {
                item.name = name;
                item.base_line = base;
            }
            ItemValue::Name(name, None) => item.name = name,
            ItemValue::UniqueId(id) => item.unique_id = id,
            ItemValue::ItemLevel(il) => item.item_lvl = il,
            ItemValue::LevelReq(lr) => item.lvl_req = lr,
            ItemValue::Sockets(s) => item.sockets = s,
            ItemValue::Quality(q) => item.quality = q,
            ItemValue::Implicits(implicits) => item.implicits = implicits,
            ItemValue::Affix(affix) => item.affixes.push(affix),
        };
    }

    Ok((i, item))
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
        assert_eq!(rarity::<()>("Rarity: RARE"), Ok(("", "RARE")));
        assert_eq!(rarity::<()>("Rarity: RARE\n"), Ok(("\n", "RARE")));
    }

    #[test]
    fn name_base_line_test() {
        assert_eq!(
            name::<()>("Loath Cut\nSmall Jewel"),
            Ok(("", ("Loath Cut", Some("Small Jewel"))))
        );
        assert_eq!(
            name::<()>("Mega Flask\nSome affix"),
            Ok(("\nSome affix", ("Mega Flask", None)))
        );
    }

    #[test]
    fn implicits_count_test() {
        assert_eq!(implicits::<()>("Implicits: 0"), Ok(("", vec![])));
        assert_eq!(
            implicits::<()>("Implicits: 2\nAffix1\nAffix2"),
            Ok(("", vec!["Affix1", "Affix2"]))
        );
        assert_eq!(
            implicits::<()>(&"Implicits: no"),
            Err(nom::Err::Failure(()))
        );
    }

    #[test]
    fn affix_test() {
        assert_eq!(
            affix::<()>("{crafted}Adds 2 Passive Skills"),
            Ok(("", "Adds 2 Passive Skills"))
        );
        assert_eq!(
            affix::<()>("Added Small Passive Skills also grant: +3% to Chaos Resistance\n"),
            Ok((
                "\n",
                "Added Small Passive Skills also grant: +3% to Chaos Resistance"
            ))
        );

        assert_eq!(affix::<()>(""), Ok(("", "")));
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
1 Added Passive Skill is Elegant Form"#;

        let (_, item) = parse_pob_item::<()>(&item)?;
        assert_eq!(item.rarity, "RARE");
        assert_eq!(item.name, "Loath Cut");
        assert_eq!(item.base_line, "Small Cluster Jewel");
        assert_eq!(
            item.unique_id,
            "c9ec1ff43acb2852474f462ce952d771edbf874f9710575a9e9ebd80b6e6dbfb"
        );
        assert_eq!(item.item_lvl, 84);
        assert_eq!(item.lvl_req, 54);
        assert_eq!(
            item.implicits,
            vec![
                "Adds 2 Passive Skills",
                "Added Small Passive Skills grant: 1% chance to Dodge Attack Hits"
            ]
        );
        assert_eq!(item.affixes.len(), 4);
        Ok(())
    }

    #[test]
    fn pob_item1() -> Result<(), anyhow::Error> {
        use nom::error::VerboseError;

        let item = r#"
        			Rarity: MAGIC
Experimenter's Silver Flask of Adrenaline
Unique ID: c923e98f2fa95e0c18b019f4e203137ea0c17c35e01273c53ccbef8324125ac4
Item Level: 53
Quality: 0
LevelReq: 22
Implicits: 0
21% increased Movement Speed during Flask effect
38% increased Duration"#;

        let (_, _) = parse_pob_item::<VerboseError<&str>>(&item)?;
        Ok(())
    }
}
