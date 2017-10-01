use std::thread;
use std::time::Duration;

use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use reqwest;
use reqwest::Client;
use feed_rs::parser;
use feed_rs::feed::{Feed as RssFeed};

use errors::*;
use mindstream::feeds::{is_feed_exist, insert_feed, Feed};
use mindstream::sources::{Source, SourceOption, RssSource, find_rss_sources};
use mindstream::users_sources::find_users_by_source;
use mindstream::users_feeds::{UserFeed, Reaction, is_user_feed_already_inserted};
use mindstream::mercury::{fetch_readable};
use users::User;
use pg::PgDatabase;

pub fn fetch_feeds_channel(url: &str) -> Result<Option<RssFeed>> {
    let mut response = reqwest::get(url)?;
    let feed = parser::parse(&mut response);
    Ok(feed)
}

pub fn run_rss_job(client: Client, pool: Pool<PostgresConnectionManager>) {
    thread::spawn(move || {
        loop {
            if let Err(err) = process_feeds(&client, &pool) {
                println!("process_rss error {:?}", err);
            }
            let duration = Duration::from_secs(1 * 60);
            thread::sleep(duration);
        }
    });
}

fn process_feeds(client: &Client, pool: &Pool<PostgresConnectionManager>) -> Result<()> {
    let conn = pool.get()?;
    let pg = PgDatabase::new(conn);
    let sources = find_rss_sources(&pg, i32::max_value(), 0)?;
    for source in sources {
        let subscribers = find_users_by_source(&pg, &source)?;
        match source.options()? {
            SourceOption::Rss(rss_source) => {
                process_rss_source(&subscribers, &source, &rss_source, client, &pg)?;
            },
            SourceOption::Twitter(_) => {}
        }
    }
    Ok(())
}

fn process_rss_source(subscribers: &Vec<User>, source: &Source, rss_source: &RssSource, client: &Client, pg: &PgDatabase) -> Result<()> {
    if let Some(feeds_channel) = fetch_feeds_channel(&rss_source.xml_url)? {
        for rss_feed in &feeds_channel.entries {
            for link in &rss_feed.alternate {
                if !is_feed_exist(&pg, &link.href, source)? {
                    let readable = {
                        match fetch_readable(client, &link.href) {
                            Ok(r) => r,
                            Err(_) => None,
                        }
                    };
                    let feed = Feed::new(&link.href, Some(rss_feed.clone().into()), readable, None, source.uuid);
                    if insert_feed(&pg, &feed).is_ok() {
                        println!("readable inserted {:?} from {:?}", feed.url, &rss_source.xml_url);
                        insert_subscribers_feeds(subscribers, &feed, pg)?;
                    }
                }
            }
        }
    }
    Ok(())
}

fn insert_subscribers_feeds(subscribers: &Vec<User>, feed: &Feed, pg: &PgDatabase) -> Result<()> {
    for subscriber in subscribers {
        let user_feed = UserFeed::new(subscriber.uuid, feed.uuid.clone(), Reaction::Unreaded);
        if !is_user_feed_already_inserted(pg, &feed.url, &subscriber)? {
            if pg.insert(&user_feed).is_ok() {
                println!("insert subscriber {:?} -> {:?}", &feed.url, subscriber.uuid);
            }
        }
    }
    Ok(())
}