#[macro_use]
extern crate diesel;

#[cfg(feature = "sentry")]
use sentry_ as sentry;

use scraper::{Html, Selector};

use std::env;

use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

mod fcm;
mod models;
mod schema;
use self::models::Item;
mod parsers;

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("failed to fetch site correctly: {0}")]
    FetchFailed(String),
    #[error("failed to parse and select the html")]
    ParseFailed,
    #[error("item not found in database")]
    ItemNotFound,
}

#[derive(Debug)]
struct CapturedError(anyhow::Error);
impl<T: Into<anyhow::Error>> From<T> for CapturedError {
    fn from(t: T) -> CapturedError {
        let e = t.into();

        #[cfg(feature = "sentry")]
        sentry::integrations::anyhow::capture_anyhow(&e);

        CapturedError(e)
    }
}

#[async_std::main]
async fn main() -> anyhow::Result<(), CapturedError> {
    use schema::item::dsl as table;
    dotenv::dotenv()?; // Need to load .env blocking, because will not be used as returned value.

    #[cfg(feature = "sentry")]
    let _guard = sentry::init(env::var("SENTRY_DSN")?);

    let dom = Html::parse_document({
        let start = std::time::Instant::now();
        let mut resp = surf::get("http://steamrmt.com/skinbuy.html")
            .set_header("User-Agent", &env::var("USER_AGENT")?)
            .await
            .map_err(|e| anyhow::anyhow!(e))?;

        let status = resp.status();
        if status != 200 {
            Err(Error::FetchFailed(status.to_string()))?
        }
        println!(
            "Fetched site with status `{}` in {:?}",
            status,
            start.elapsed()
        );

        &resp.body_string().await.map_err(|e| anyhow::anyhow!(e))?
    });
    println!("Successfully parsed!");

    let sel = Selector::parse("html > body > div.contents > div.inner > div.main > section")
        .map_err(|_| Error::ParseFailed)?;
    let s = dom.select(&sel).next().unwrap();
    let lines = s.text();

    // Connect to the MySQL!
    let conn = MysqlConnection::establish(&env::var("DATABASE_URL")?)?;

    let fcm_client = fcm::Client::new(
        &env::var("FCM_SERVER_KEY")?,
        &env::var("FCM_REGISTRATION_ID")?,
    );

    let mut notifications = vec![];

    let mut order_ids = Vec::<i32>::new();
    let mut new_items = Vec::<Item>::new();

    for found_item in parsers::parse_items(lines) {
        order_ids.push(found_item.order_id);

        let db_item: Option<Item> = table::item
            .filter(table::order_id.eq(found_item.order_id))
            .first(&conn)
            .optional()?;

        if let Some(db_item) = db_item {
            if let Some(found_item) = found_item.item {
                if found_item.price != db_item.price {
                    // price changed
                    notifications.push(fcm_client.send_notification(build_notification! {
                        title = format!("{} の価格が変更されました", found_item);
                        body = format!("{} 円から {} 円になりました。", db_item.price, found_item.price);
                    }));
                }

                diesel::update(&found_item)
                    .set(&found_item)
                    .execute(&conn)?;
            } else if !db_item.has_sold {
                notifications.push(fcm_client.send_notification(build_notification! {
                    title = format!("{} が売約済みになりました", db_item);
                }));

                diesel::update(&db_item)
                    .set((table::has_sold.eq(true), table::price.eq(found_item.price)))
                    .execute(&conn)?;
            }
        } else {
            if let Some(found_item) = found_item.item {
                notifications.push(fcm_client.send_notification(build_notification! {
                    title = format!("{} が新たに追加されました", found_item);
                }));

                new_items.push(found_item);
            } else {
                // 売却済みかつDBにも情報がない場合、何もしない
            }
        }
    }

    if new_items.len() > 0 {
        diesel::insert_into(table::item)
            .values(new_items)
            .execute(&conn)?;
    }

    // dbにはあるけどサイトにはないアイテムのIDリストを取得する
    let mut deleted_items = table::item.select(table::order_id).load::<i32>(&conn)?;
    deleted_items.retain(|x| !order_ids.contains(x));

    if deleted_items.len() > 0 {
        for id in deleted_items {
            let item = table::item
                .filter(table::order_id.eq(id))
                .first::<Item>(&conn)
                .optional()?;
            diesel::delete(table::item.filter(table::order_id.eq(id))).execute(&conn)?;
            if let Some(item) = item {
                notifications.push(fcm_client.send_notification(build_notification! {
                    title = format!("{} が削除されました", item);
                }));
            } else {
                Err(Error::ItemNotFound)?
            }
        }
    }

    if notifications.len() > 0 {
        println!("Sending {} notification(s)...", notifications.len());
        futures::future::join_all(notifications).await;
        println!("{}", "Sent!");
    }

    Ok(())
}
