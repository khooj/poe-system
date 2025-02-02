use domain::{Category, Item, Mod, ModType, BASE_TYPES};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{
        alpha1, alphanumeric1, digit1, line_ending, multispace0, not_line_ending,
    },
    combinator::{cut, map, map_res, opt},
    error::{context, ContextError, FromExternalError, ParseError},
    multi::{length_count, many0, many_m_n},
    sequence::{delimited, preceded, tuple},
    IResult,
};

use std::num::ParseIntError;
use std::str::FromStr;

pub(crate) struct ParsedItem {
    pub item: Item,
}

#[derive(Debug, PartialEq)]
pub(crate) enum ItemValue<'a> {
    Rarity(&'a str),
    BaseType { name: String, base: String },
    ItemLevel(i32),
    LevelReq(i32),
    UniqueId(&'a str),
    Affix((&'a str, ModType, bool)),
    Implicits(Vec<(&'a str, bool)>),
    Quality(i32),
    Sockets(String),
    // Influence(String),
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

fn implicits<'a, E>(i: &'a str) -> IResult<&'a str, Vec<(&'a str, bool)>, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, ParseIntError> + ContextError<&'a str>,
{
    context(
        "implicits",
        length_count(
            map(preceded(tag("Implicits: "), cut(digit1)), |out: &str| {
                usize::from_str(out).unwrap_or(0usize)
            }),
            map(
                |i| affix(i, ModType::Implicit),
                |(t, _, crafted)| (t, crafted),
            ),
        ),
    )(i)
}

fn affix_prefixes<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, bool, E> {
    context(
        "affix_prefixes",
        map(
            alt((
                tag("Prefix: "),
                tag("Suffix: "),
                tag("Crafted: "),
                tag("{crafted}"),
                tag("{range:1}"),
            )),
            |t: &str| ["{crafted}", "Crafted: "].contains(&t),
        ),
    )(i)
}

fn affix<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
    default: ModType,
) -> IResult<&'a str, (&'a str, ModType, bool), E> {
    context(
        "affix",
        map(
            preceded(
                multispace0,
                tuple((opt(affix_prefixes), cut(not_line_ending))),
            ),
            |(crafted, t)| (t, default, crafted.unwrap_or_default()),
        ),
    )(i)
}

fn item_value<
    'a,
    E: ParseError<&'a str> + FromExternalError<&'a str, ParseIntError> + ContextError<&'a str>,
>(
    i: &'a str,
) -> IResult<&'a str, ItemValue<'a>, E> {
    context(
        "item_value",
        alt((
            map(unique_id, ItemValue::UniqueId),
            map(item_lvl, ItemValue::ItemLevel),
            map(level_req, ItemValue::LevelReq),
            map(implicits, ItemValue::Implicits),
            map(quality, ItemValue::Quality),
            map(sockets, |s| ItemValue::Sockets(String::from(s))),
            map(|i| affix(i, ModType::Explicit), ItemValue::Affix),
        )),
    )(i)
}

fn root<
    'a,
    E: ParseError<&'a str> + FromExternalError<&'a str, ParseIntError> + ContextError<&'a str>,
>(
    i: &'a str,
) -> IResult<&'a str, Vec<ItemValue>, E> {
    let (i, rarity) = delimited(multispace0, rarity, line_ending)(i)?;
    let (i, basetype) = delimited(multispace0, name, line_ending)(i)?;
    // let (i, mut header_vals) =
    //     many_m_n(2, 2, delimited(multispace0, item_value_header, line_ending))(i)?;
    let (i, mut vals) = many0(delimited(multispace0, item_value, line_ending))(i)?;
    let (i, end_val) = item_value(i)?;
    let mut values = vec![ItemValue::Rarity(rarity), basetype];
    values.append(&mut vals);
    values.push(end_val);
    Ok((i, values))
}

pub(crate) fn parse_pob_item<'a, E>(i: &'a str) -> IResult<&'a str, ParsedItem, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, ParseIntError> + ContextError<&'a str>,
{
    let (i, items) = root(i)?;
    let mut item = Item::default();
    let mut mods = vec![];
    let mut category = Category::Maps;

    for val in items {
        match val {
            ItemValue::Rarity(r) => item.rarity = r.to_string(),
            ItemValue::BaseType { base, name } => {
                category = Category::parse_from_basetype(base.replace(" ", "")).unwrap_or_default();
                item.name = name;
                item.base_type = base;
            }
            ItemValue::UniqueId(id) => item.id = id.to_string(),
            ItemValue::ItemLevel(il) => item.item_lvl = domain::ItemLvl::Yes(il),
            ItemValue::LevelReq(lr) => item.lvl_req = lr,
            ItemValue::Sockets(s) => item.sockets = s,
            ItemValue::Quality(q) => item.quality = q,
            ItemValue::Implicits(implicits) => {
                mods.extend(implicits.into_iter().map(|im| (im.0, ModType::Implicit)))
            }
            ItemValue::Affix(e) => mods.push((e.0, e.1)),
            _ => {}
        };
    }

    let mods = Mod::many_by_stat(&mods);

    item.mods = mods;
    item.category = category;

    Ok((i, ParsedItem { item }))
}

#[cfg(test)]
mod test {
    use domain::Mod;
    use nom::error::VerboseError;

    use super::*;

    macro_rules! gen_test {
        ($name:ident, $f:ident, $data:expr, $res:expr) => {
            #[test]
            fn $name() -> anyhow::Result<()> {
                let (_, res) = $f::<()>($data)?;
                assert_eq!(res, $res);
                Ok(())
            }
        };
    }

    gen_test!(
        name_check,
        name,
        "Loath Cut\nSmall Cluster Jewel",
        ItemValue::BaseType {
            name: "Loath Cut".to_string(),
            base: "Small Cluster Jewel".to_string()
        }
    );
    gen_test!(quality_check, quality, "Quality: 21", 21);
    gen_test!(rarity_check, rarity, "Rarity: RARE", "RARE");
    gen_test!(uniqueid_check, unique_id, "Unique ID: asd", "asd");
    gen_test!(sockets_check, sockets, "Sockets: asd", "asd");
    gen_test!(itemlvl_check, item_lvl, "Item Level: 21", 21);
    gen_test!(levelreq_check, level_req, "LevelReq: 21", 21);

    // TODO: add checks
    gen_test!(itemvalue_qual_check, item_value, "Quality: 21", ItemValue::Quality(21));

    #[test]
    fn affix_check() -> anyhow::Result<()> {
        for data in ["Prefix: asd", "Suffix: asd", "{range:1}asd"] {
            let (_, affixes) = affix::<()>(data, ModType::Explicit)?;
            assert_eq!(affixes, ("asd", ModType::Explicit, false));
        }

        for data in ["Crafted: asd", "{crafted}asd"] {
            let (_, affixes) = affix::<()>(data, ModType::Explicit)?;
            assert_eq!(affixes, ("asd", ModType::Explicit, true));
        }
        Ok(())
    }

    #[test]
    fn implicits_check() -> Result<(), anyhow::Error> {
        let i = "Implicits: 1\nAdds 2 Passive Skills\nAdds 3 Passive Skills";
        let (i, ret) = implicits::<VerboseError<&str>>(i)?;
        assert_eq!(i, "\nAdds 3 Passive Skills");
        assert_eq!(ret, vec![("Adds 2 Passive Skills", false)]);

        let i = "Implicits: 0\nAdds 2 Passive Skills\nAdds 3 Passive Skills";
        let (i, ret) = implicits::<VerboseError<&str>>(i)?;
        assert_eq!(i, "\nAdds 2 Passive Skills\nAdds 3 Passive Skills");
        assert!(ret.is_empty());
        Ok(())
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

        let (_, item) = parse_pob_item::<VerboseError<&str>>(item)?;
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

    #[test]
    #[ignore]
    fn parse_category_by_basetype() -> anyhow::Result<()> {
        let text = r#"
			Rarity: RARE
Behemoth Spark
Slink Boots
Unique ID: 2a2a31fe43e29e6ee51bfc386bb5a6ab6bfd083484980247e3d35d5504a965ca
Hunter Item
Item Level: 86
Quality: 0
Sockets: B-G-R-B
LevelReq: 69
Implicits: 0
+58 to Evasion Rating
+59 to maximum Life
+34% to Chaos Resistance
35% increased Movement Speed
You have Tailwind if you have dealt a Critical Strike Recently
{crafted}24% reduced Effect of Chill and Shock on you
"#;
        let (_, item) = parse_pob_item::<VerboseError<&str>>(text)?;
        assert_eq!(item.item.category, Category::Armour);
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
