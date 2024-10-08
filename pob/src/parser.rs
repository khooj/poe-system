use domain::{Item, Mod, ModType, BASE_TYPES};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{
        alpha1, alphanumeric1, digit1, line_ending, multispace0, not_line_ending,
    },
    combinator::{cut, map, map_res, opt},
    error::{context, ContextError, FromExternalError, ParseError},
    multi::{length_count, many0, many_m_n},
    sequence::{delimited, preceded},
    IResult,
};

use std::num::ParseIntError;
use std::str::FromStr;

pub(crate) struct ParsedItem {
    pub item: Item,
}

#[derive(Debug, PartialEq)]
pub(crate) enum ItemValue<'a> {
    Rarity(String),
    BaseType { name: String, base: String },
    ItemLevel(i32),
    LevelReq(i32),
    UniqueId(String),
    Affix((&'a str, ModType)),
    Implicits(Vec<ItemValue<'a>>),
    Quality(i32),
    Sockets(String),
    // Influence(String),
    SkipLine,
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

    for itemclass in BASE_TYPES.iter() {
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

    let (i, basetype) = preceded(line_ending, base_type)(i)?;
    Ok((
        i,
        ItemValue::BaseType {
            base: basetype.to_string(),
            name: name.to_string(),
        },
    ))
}

fn base_type<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    context("base_type", cut(not_line_ending))(i)
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
    context(
        "implicits_count",
        length_count(
            map(preceded(tag("Implicits: "), cut(digit1)), |out: &str| {
                usize::from_str(out).unwrap_or(0usize)
            }),
            preceded(alt((multispace0, line_ending)), |i| {
                affix(i, ModType::Implicit)
            }),
        ),
    )(i)
}

fn skip_line_with_tag<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
    t: &'a str,
) -> IResult<&'a str, ItemValue<'a>, E> {
    context(
        "skip_line_tag",
        map(preceded(tag(t), cut(not_line_ending)), |_| {
            ItemValue::SkipLine
        }),
    )(i)
}

fn affix_internal<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
    default: ModType,
) -> IResult<&'a str, ItemValue<'a>, E> {
    context(
        "affix_internal",
        map(
            preceded(
                opt(alt((tag("{crafted}"), tag("{range:1}")))),
                cut(not_line_ending),
            ),
            |e: &str| ItemValue::Affix((e, default)),
        ),
    )(i)
}

fn affix<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
    default: ModType,
) -> IResult<&'a str, ItemValue, E> {
    context(
        "affix",
        preceded(
            multispace0,
            alt((
                |i| skip_line_with_tag(i, "Prefix: "),
                |i| skip_line_with_tag(i, "Suffix: "),
                |i| skip_line_with_tag(i, "Crafted: "),
                |i| affix_internal(i, default),
            )),
        ),
    )(i)
}

fn item_value_header<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, ItemValue, E> {
    context(
        "item_value_header",
        alt((map(rarity, |s| ItemValue::Rarity(String::from(s))), name)),
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
        alt((
            map(unique_id, |s| ItemValue::UniqueId(String::from(s))),
            map(item_lvl, ItemValue::ItemLevel),
            map(level_req, ItemValue::LevelReq),
            map(implicits, ItemValue::Implicits),
            map(quality, ItemValue::Quality),
            map(sockets, |s| ItemValue::Sockets(String::from(s))),
            |i| affix(i, ModType::Explicit),
        )),
    )(i)
}

fn root<
    'a,
    E: ParseError<&'a str> + FromExternalError<&'a str, ParseIntError> + ContextError<&'a str>,
>(
    i: &'a str,
) -> IResult<&'a str, Vec<ItemValue>, E> {
    let (i, mut header_vals) =
        many_m_n(2, 2, delimited(multispace0, item_value_header, line_ending))(i)?;
    let (i, mut vals) = many0(delimited(multispace0, item_value, line_ending))(i)?;
    let (i, end_val) = item_value(i)?;
    header_vals.append(&mut vals);
    header_vals.push(end_val);
    Ok((i, header_vals))
}

pub(crate) fn parse_pob_item<'a, E>(i: &'a str) -> IResult<&'a str, ParsedItem, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, ParseIntError> + ContextError<&'a str>,
{
    let (i, items) = root(i)?;
    let mut item = Item::default();
    let mut mods = vec![];

    for val in items {
        match val {
            ItemValue::Rarity(r) => item.rarity = r,
            ItemValue::BaseType { base, name } => {
                item.name = name;
                item.base_type = base;
            }
            ItemValue::UniqueId(id) => item.id = id,
            ItemValue::ItemLevel(il) => item.item_lvl = domain::ItemLvl::Yes(il),
            ItemValue::LevelReq(lr) => item.lvl_req = lr,
            ItemValue::Sockets(s) => item.sockets = s,
            ItemValue::Quality(q) => item.quality = q,
            ItemValue::Implicits(implicits) => mods.extend(implicits.into_iter().map(|e| {
                if let ItemValue::Affix(..) = e {
                    e
                } else {
                    unreachable!()
                }
            })),
            e @ ItemValue::Affix(..) => mods.push(e),
            _ => {}
        };
    }

    let mods = mods
        .into_iter()
        .map(|m| match m {
            ItemValue::Affix((s, t)) => (s, t),
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();

    let mods = Mod::many_by_stat(&mods);

    item.mods = mods;

    Ok((i, ParsedItem { item }))
}

#[cfg(test)]
mod test {
    use domain::Mod;

    use super::*;

    #[test]
    fn name_check() -> Result<(), anyhow::Error> {
        dotenv::dotenv().ok();
        let i = "Loath Cut\nSmall Cluster Jewel";
        let (_, ret) = name::<()>(i)?;
        assert_eq!(
            ret,
            ItemValue::BaseType {
                base: "Small Cluster Jewel".to_string(),
                name: "Loath Cut".to_string()
            }
        );
        Ok(())
    }

    #[test]
    fn implicits_check() -> Result<(), anyhow::Error> {
        use nom::error::VerboseError;
        let i = "Implicits: 1\nAdds 2 Passive Skills\nAdds 3 Passive Skills";
        let (i, ret) = implicits::<VerboseError<&str>>(i)?;
        assert_eq!(i, "\nAdds 3 Passive Skills");
        assert_eq!(
            ret,
            vec![ItemValue::Affix((
                "Adds 2 Passive Skills",
                ModType::Implicit
            ))]
        );

        let i = "Implicits: 0\nAdds 2 Passive Skills\nAdds 3 Passive Skills";
        let (i, ret) = implicits::<VerboseError<&str>>(i)?;
        assert_eq!(i, "\nAdds 2 Passive Skills\nAdds 3 Passive Skills");
        assert!(ret.is_empty());
        Ok(())
    }

    #[test]
    fn simple_pob_item() -> Result<(), anyhow::Error> {
        use nom::error::VerboseError;
        dotenv::dotenv().ok();
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

        let (_, item) = parse_pob_item::<VerboseError<&str>>(&item)?;
        assert_eq!(item.item.rarity, "RARE");
        assert_eq!(item.item.name, "Loath Cut");
        assert_eq!(item.item.base_type, "Small Cluster Jewel");
        assert_eq!(
            item.item.id,
            "c9ec1ff43acb2852474f462ce952d771edbf874f9710575a9e9ebd80b6e6dbfb"
        );
        assert_eq!(item.item.item_lvl, domain::ItemLvl::Yes(84));
        assert_eq!(item.item.lvl_req, 54);
        assert_eq!(
            item.item.mods,
            Mod::many_by_stat_or_invalid(&[
                ("Adds 2 Passive Skills", ModType::Implicit),
                (
                    "Added Small Passive Skills grant: 1% chance to Dodge Attack Hits",
                    ModType::Implicit
                ),
                (
                    "Added Small Passive Skills also grant: +3% to Chaos Resistance",
                    ModType::Explicit
                ),
                (
                    "Added Small Passive Skills also grant: +5 to Maximum Energy Shield",
                    ModType::Explicit
                ),
                (
                    "Added Small Passive Skills also grant: +5 to Strength",
                    ModType::Explicit
                ),
                ("1 Added Passive Skill is Elegant Form", ModType::Explicit),
            ])
        );
        Ok(())
    }

    // #[test]
    // fn pob_item1() -> Result<(), anyhow::Error> {
    //     use nom::error::VerboseError;
    //     dotenv::dotenv().ok();

    //     let item = r#"
    //         			Rarity: MAGIC
    // Experimenter's Silver Flask of Adrenaline
    // Unique ID: c923e98f2fa95e0c18b019f4e203137ea0c17c35e01273c53ccbef8324125ac4
    // Item Level: 53
    // Quality: 0
    // LevelReq: 22
    // Implicits: 0
    // 21% increased Movement Speed during Flask effect
    // 38% increased Duration"#;

    //     let (_, item) = parse_pob_item::<VerboseError<&str>>(&item)?;
    //     assert_eq!(item.rarity, "MAGIC");
    //     assert_eq!(item.name, "Experimenter's Silver Flask of Adrenaline");
    //     assert_eq!(item.base_type, "Silver Flask");
    //     assert_eq!(
    //         item.id,
    //         "c923e98f2fa95e0c18b019f4e203137ea0c17c35e01273c53ccbef8324125ac4"
    //     );
    //     assert_eq!(item.item_lvl, domain::ItemLvl::Yes(53));
    //     assert_eq!(item.lvl_req, 22);
    //     assert_eq!(
    //         item.mods,
    //         Mod::many_by_stat_or_invalid(&LinkedList::from_iter(
    //             [
    //                 (
    //                     "21% increased Movement Speed during Flask effect",
    //                     ModType::Explicit
    //                 ),
    //                 ("38% increased Duration", ModType::Explicit),
    //             ]
    //             .into_iter()
    //         )),
    //     );

    //     Ok(())
    // }
}
