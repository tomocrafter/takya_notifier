use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::iter::Iterator;

use super::models::{Exterior, Item};

const STATTRAK: &str = "StatTrak ";

lazy_static! {
    static ref SOLD_MATCHER: Regex = Regex::new(r"\(売約済み\) #(\d+)").unwrap();
    static ref VANILLA_MATCHER: Regex = Regex::new(r"([A-Za-z ]+) \(Vanilla\) #(\d+)").unwrap();
    static ref ITEM_MATCHER: Regex = Regex::new(r"([A-Za-z ]+) \(([-A-Za-z ]+)\) #(\d+)").unwrap();
    static ref PRICE_MATCHER: Regex = Regex::new(r"販売価格: ([0-9,]+)円 *").unwrap();
}

pub struct ItemSection {
    // If item has already sold, then item may be None.
    pub item: Option<Item>,
    pub order_id: i32,
    pub price: i32,
}

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("invalid item format (expected `name | kind (exterior) #id` or `name (Vanilla) #id` or `(売約済み) #id`, found `{0}`)")]
    InvalidItemFormat(String),
    #[error(transparent)]
    InvalidNumber(#[from] std::num::ParseIntError),
    #[error("invalid exterior (expected `FN`, `MW`, `FT`, `WW` or `BS` found `{0}`)")]
    InvalidExterior(String),
}

#[inline]
fn warn_corrupted_section<I: std::fmt::Display>(why: I) {
    eprintln!("Warn: found corrupted item section, {}", why);
}

// Consumes lines iterator
pub fn parse_items<'a>(mut lines: scraper::element_ref::Text) -> Vec<ItemSection> {
    let mut items = vec![];

    while let Some(t) = lines.next() {
        if t.trim() == "★" {
            // Found item section
            // then parse it!

            // find item name line
            let item_name_line = match lines.next() {
                Some(line) => line,
                None => {
                    warn_corrupted_section("no item name line found.");
                    continue;
                }
            };

            // Discard blank line.
            if lines.next().is_none() {
                warn_corrupted_section("no blank line found.");
                continue;
            }

            // find price line
            let price_line = match lines.next() {
                Some(line) => line,
                None => {
                    warn_corrupted_section("no price line found.");
                    continue;
                }
            };

            let item = match parse_item_section(item_name_line, price_line) {
                Ok(item) => item,
                Err(e) => {
                    warn_corrupted_section(e);
                    continue;
                }
            };

            items.push(item);
        }
    }

    items
}

pub fn parse_item_section<'a>(
    item_name_line: &'a str,
    price_line: &'a str,
) -> Result<ItemSection, ParseError> {
    let mut name: Option<String> = None;
    let mut kind: Option<String> = None;
    let mut exterior: Option<Exterior> = None;
    let order_id: i32;

    // Parse for item name, skin, order number.
    let v = item_name_line.split(" | ").collect::<Vec<&str>>();
    match v.len() {
        1 => {
            // Vanilla Item or Sold
            if let Some(sold_caps) = SOLD_MATCHER.captures(v[0]) {
                order_id = sold_caps[1].parse()?;
            } else {
                let caps = VANILLA_MATCHER
                    .captures(v[0])
                    .ok_or_else(|| ParseError::InvalidItemFormat(item_name_line.to_owned()))?;
                name = Some(caps[1].to_owned());
                order_id = caps[2].parse()?;
            }
        }
        2 => {
            // Normal item
            let caps = ITEM_MATCHER
                .captures(v[1])
                .ok_or_else(|| ParseError::InvalidItemFormat(item_name_line.to_owned()))?;
            name = Some(v[0].to_owned());
            kind = Some(caps[1].to_owned());

            use std::str::FromStr;
            let exterior_str = &caps[2];
            exterior = match Exterior::from_str(exterior_str) {
                Ok(exterior) => Some(exterior),
                Err(strum::ParseError::VariantNotFound) => {
                    return Err(ParseError::InvalidExterior(exterior_str.to_owned()));
                }
            };
            order_id = caps[3].parse()?;
        }
        _ => {
            return Err(ParseError::InvalidItemFormat(item_name_line.to_owned()));
        }
    }

    let trim_and_own = |t: String| t.trim().to_owned();
    name = name.map(trim_and_own);
    kind = kind.map(trim_and_own);

    // Check if item is stattrak, and remove StatTrak from name.
    let is_stattrak = {
        if let Some(n) = &mut name {
            if n.starts_with(STATTRAK) {
                n.drain(..STATTRAK.len());
                true
            } else {
                false
            }
        } else {
            false
        }
    };

    // Parse price
    let price = {
        let caps = PRICE_MATCHER.captures(price_line).unwrap();

        (caps[1]).replace(',', "").parse()?
    };

    if let Some(name) = name {
        Ok(ItemSection {
            item: Some(Item {
                order_id,
                name,
                kind,
                exterior,
                price,
                has_sold: false,
                is_stattrak,
            }),
            order_id,
            price,
        })
    } else {
        Ok(ItemSection {
            item: None,
            order_id,
            price,
        })
    }
}
