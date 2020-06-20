use diesel::{Identifiable, Insertable, Queryable};
use diesel_derive_enum::DbEnum;
use strum_macros::{AsRefStr, Display, EnumString};

use super::schema::item;

// Kind and exterior will be None if it is vanilla.
#[derive(Queryable, Insertable, Identifiable, AsChangeset)]
#[changeset_options(treat_none_as_null = "true")]
#[table_name = "item"]
#[primary_key(order_id)]
pub struct Item {
    pub order_id: i32,
    pub name: String,
    pub kind: Option<String>,
    pub exterior: Option<Exterior>,
    pub price: i32,
    pub has_sold: bool,
    pub is_stattrak: bool,
}

// For without exterior.
impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.kind {
            Some(kind) => write!(f, "{} | {}", self.name, kind),
            None => write!(f, "{} | Vanilla", self.name),
        }
    }
}

// For with exterior.
impl std::fmt::LowerExp for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.kind {
            Some(kind) => write!(
                f,
                "{} | {} ({})",
                self.name,
                kind,
                self.exterior.as_ref().unwrap()
            ),
            None => write!(f, "{} | Vanilla", self.name),
        }
    }
}

#[derive(Display, PartialEq, EnumString, Debug, AsRefStr, Clone, DbEnum)]
#[DbValueStyle = "SCREAMING_SNAKE_CASE"]
pub enum Exterior {
    #[strum(serialize = "Factory New")]
    FN,
    #[strum(serialize = "Minimal Wear")]
    MW,
    #[strum(serialize = "Field-Tested")]
    FT,
    #[strum(serialize = "Well-Worn")]
    WW,
    #[strum(serialize = "Battle-Scarred")]
    BS,
}
