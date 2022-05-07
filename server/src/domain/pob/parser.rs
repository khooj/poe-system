use crate::domain::types::{Mod, ModType};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{alpha1, alphanumeric1, digit1, line_ending, not_line_ending},
    combinator::{cut, map, map_res, opt},
    error::{context, ContextError, FromExternalError, ParseError},
    multi::{many0, many_m_n},
    sequence::{pair, preceded, terminated},
    IResult,
};
use std::num::ParseIntError;
use std::str::FromStr;
use tracing::info;

use crate::infrastructure::poe_data::BASE_ITEMS;

#[derive(Debug, PartialEq)]
enum ItemValue {
    Rarity(String),
    BaseType { base: String, name: String },
    ItemLevel(i32),
    LevelReq(i32),
    UniqueId(String),
    Affix { type_: ModType, value: String },
    Implicits(Vec<ItemValue>),
    Quality(i32),
    Sockets(String),
    Influence(String),
}

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
) -> IResult<&'a str, ItemValue, E> {
    let (i, name) = context("name", cut(not_line_ending))(i)?;

    let all_itemclasses = BASE_ITEMS.get_all_itemclasses();
    for itemclass in all_itemclasses {
        // TODO: fix?
        if itemclass.is_empty() {
            continue;
        }

        if name.contains(itemclass) {
            // its not a name, its a basetype
            return Ok((
                i,
                ItemValue::BaseType {
                    base: itemclass.to_string(),
                    name: name.to_string(),
                },
            ));
        }
    }

    let (i, basetype) = base_type(i)?;
    return Ok((
        i,
        ItemValue::BaseType {
            base: basetype.to_string(),
            name: name.to_string(),
        },
    ));
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

fn implicits<'a, E>(i: &'a str) -> IResult<&'a str, Vec<ItemValue>, E>
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
            preceded(sp, |i| affix(i, ModType::Implicit)),
        ),
    )(i)
}

fn affix<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
    default: ModType,
) -> IResult<&'a str, ItemValue, E> {
    let (i, (crafted, value)) =
        context("affix", pair(opt(tag("{crafted}")), cut(not_line_ending)))(i)?;

    Ok((
        i,
        ItemValue::Affix {
            type_: if crafted.is_some() {
                ModType::Crafted
            } else {
                default
            },
            value: value.to_string(),
        },
    ))
}

fn item_value_header<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, ItemValue, E> {
    context(
        "item_value_header",
        preceded(
            sp,
            alt((map(rarity, |s| ItemValue::Rarity(String::from(s))), name)),
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
                map(implicits, ItemValue::Implicits),
                map(quality, ItemValue::Quality),
                map(sockets, |s| ItemValue::Sockets(String::from(s))),
                |i| affix(i, ModType::Explicit),
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
    pub affixes: Vec<Mod>,
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
            ItemValue::BaseType { base, name } => {
                item.name = name;
                item.base_line = base;
            }
            ItemValue::UniqueId(id) => item.unique_id = id,
            ItemValue::ItemLevel(il) => item.item_lvl = il,
            ItemValue::LevelReq(lr) => item.lvl_req = lr,
            ItemValue::Sockets(s) => item.sockets = s,
            ItemValue::Quality(q) => item.quality = q,
            ItemValue::Implicits(implicits) => {
                item.affixes.extend(implicits.into_iter().map(|e| {
                    if let ItemValue::Affix { value, type_ } = e {
                        Mod { text: value, type_ }
                    } else {
                        Mod {
                            text: "invalid".to_string(),
                            type_: ModType::Cosmetic,
                        }
                    }
                }))
            }
            ItemValue::Affix { value, type_ } => item.affixes.push(Mod { text: value, type_ }),
            _ => {
                todo!()
            }
        };
    }

    Ok((i, item))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn name_check() -> Result<(), anyhow::Error> {
        dotenv::dotenv().ok();
        let i = "Loath Cut\nSmall Cluster Jewel";
        let (_, ret) = name::<()>(i)?;
        assert_eq!(ret, ItemValue::BaseType{ base: "Small Cluster Jewel".to_string(), name: "Loath Cut".to_string()});
        Ok(())
    }

    #[test]
    fn simple_pob_item() -> Result<(), anyhow::Error> {
        dotenv::dotenv().ok();
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
            item.affixes,
            vec![
                Mod::from_str_type("Adds 2 Passive Skills", ModType::Crafted),
                Mod::from_str_type(
                    "Added Small Passive Skills grant: 1% chance to Dodge Attack Hits",
                    ModType::Crafted
                ),
                Mod::from_str_type(
                    "Added Small Passive Skills also grant: +3% to Chaos Resistance",
                    ModType::Explicit
                ),
                Mod::from_str_type(
                    "Added Small Passive Skills also grant: +5 to Maximum Energy Shield",
                    ModType::Explicit
                ),
                Mod::from_str_type(
                    "Added Small Passive Skills also grant: +5 to Strength",
                    ModType::Explicit
                ),
                Mod::from_str_type("1 Added Passive Skill is Elegant Form", ModType::Explicit),
            ]
        );
        Ok(())
    }

    #[test]
    fn pob_item1() -> Result<(), anyhow::Error> {
        use nom::error::VerboseError;
        dotenv::dotenv().ok();

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

        let (_, item) = parse_pob_item::<VerboseError<&str>>(&item)?;
        assert_eq!(item.rarity, "MAGIC");
        assert_eq!(item.name, "Experimenter's Silver Flask of Adrenaline");
        assert_eq!(item.base_line, "Silver Flask");
        assert_eq!(
            item.unique_id,
            "c923e98f2fa95e0c18b019f4e203137ea0c17c35e01273c53ccbef8324125ac4"
        );
        assert_eq!(item.item_lvl, 53);
        assert_eq!(item.lvl_req, 22);
        assert_eq!(
            item.affixes,
            vec![
                Mod::from_str_type(
                    "21% increased Movement Speed during Flask effect",
                    ModType::Explicit
                ),
                Mod::from_str_type("38% increased Duration", ModType::Explicit),
            ]
        );

        Ok(())
    }
}
