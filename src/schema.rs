table! {
    use diesel::sql_types::*;
    use crate::models::ExteriorMapping;

    item (order_id) {
        order_id -> Integer,
        name -> Varchar,
        kind -> Nullable<Varchar>,
        exterior -> Nullable<ExteriorMapping>,
        price -> Integer,
        has_sold -> Bool,
        is_stattrak -> Bool,
    }
}
