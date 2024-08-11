use core::cmp::Ordering;
use poeninja::models::Object;
use std::collections::HashMap;
use tradeapi::models::ClientFetchItem;

pub struct Ultimatum {
    pub sacrifice: String,
    pub reward: String,
    pub div: bool,
    pub count: String,
    pub currency: bool,
}

#[derive(serde::Deserialize)]
pub struct Property {
    pub name: String,
    pub values: Vec<Vec<serde_json::Value>>,
}

pub fn map_ultimatum(item: ClientFetchItem) -> Ultimatum {
    let vv: Vec<Property> = serde_json::from_value(item.rest["properties"].clone()).unwrap();
    let v: Vec<String> = vv
        .into_iter()
        .map(|e| (e.name, e.values))
        .filter(|e| e.0.contains("Requires Sacrifice") || e.0.contains("Reward"))
        .flat_map(|e| {
            if e.1.len() == 2 {
                vec![
                    e.1[0][0].as_str().unwrap().to_string(),
                    e.1[1][0].as_str().unwrap().to_string(),
                ]
            } else {
                vec![e.1[0][0].as_str().unwrap().to_string()]
            }
        })
        .collect();

    let div =
        v[1].contains("Divination") || v.get(2).unwrap_or(&String::new()).contains("Divination");
    let currency =
        v[1].contains("Currency") || v.get(2).unwrap_or(&String::new()).contains("Currency");
    let (sacrifice, reward, count) = if div || currency {
        let v0 = v[0].clone();
        let v1 = v[1].clone();
        let v2 = v[2].clone();
        (v0, v2, v1)
    } else {
        (v[0].clone(), v[1].clone(), String::new())
    };

    Ultimatum {
        sacrifice,
        reward,
        div,
        count,
        currency,
    }
}

#[derive(PartialEq)]
pub enum Price {
    Range { min: f32, max: f32 },
    Static(f32),
}

impl PartialOrd for Price {
    fn partial_cmp(&self, other: &Price) -> Option<Ordering> {
        Some(match self {
            Price::Range { max, .. } => match other {
                Price::Range { max: max2, .. } if max > max2 => Ordering::Greater,
                Price::Static(val) if max > val => Ordering::Greater,
                _ => Ordering::Less,
            },
            Price::Static(val) => match other {
                Price::Range { max, .. } if val > max => Ordering::Greater,
                Price::Static(val2) if val > val2 => Ordering::Greater,
                _ => Ordering::Less,
            },
        })
    }
}

impl std::fmt::Display for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Price::Range { min, max } => write!(f, "{} - {}", min, max),
            Price::Static(val) => write!(f, "{}", val),
        }
    }
}

pub struct Data {
    pub chaos_price: Price,
    pub div_price: Price,
}

pub fn get_price<T>(v: &Vec<Object>, f: T) -> Price
where
    T: Fn(&Object) -> f32,
{
    let min = v.iter().map(|e| f(e)).reduce(f32::min).unwrap();
    let max = v.iter().map(f).reduce(f32::max).unwrap();
    if min != max {
        Price::Range { min, max }
    } else {
        Price::Static(min)
    }
}

pub fn process_poeninja_resp(items: Vec<Object>) -> HashMap<String, Data> {
    let grouped = items.into_iter().fold(HashMap::new(), |mut acc, x| {
        let v = acc.entry(x.name.clone()).or_insert(vec![]);
        v.push(x);
        acc
    });

    let mut res = HashMap::new();
    for (k, v) in grouped {
        let chaos_price = get_price(&v, |e| e.chaos_value);
        let div_price = get_price(&v, |e| e.divine_value);
        res.insert(
            k,
            Data {
                chaos_price,
                div_price,
            },
        );
    }
    res
}

pub struct EveryNElements<T> {
    n: usize,
    iter: T,
}

impl<T> EveryNElements<T> {
    pub fn new(n: usize, iter: T) -> Self {
        EveryNElements { n, iter }
    }
}

impl<T, K> Iterator for EveryNElements<T>
where
    T: Iterator<Item = K>,
{
    type Item = Vec<K>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut v = Vec::with_capacity(self.n);
        for _ in 0..self.n {
            let i = self.iter.next();
            if i.is_some() {
                v.push(i.unwrap());
            } else {
                break;
            }
        }
        if v.is_empty() {
            None
        } else {
            Some(v)
        }
    }
}
