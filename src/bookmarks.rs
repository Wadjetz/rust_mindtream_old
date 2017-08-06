use uuid::Uuid;
use chrono::NaiveDateTime;
use chrono::prelude::*;
use postgres::rows::Row;
use postgres_shared::types::ToSql;
use juniper::Executor;

use errors::*;
use graphql::query::Query;
use users::User;
use pg::{Insertable, PgDatabase};

#[derive(Debug)]
pub struct Bookmark {
    pub uuid: Uuid,
    pub url: String,
    pub title: String,
    pub description: Option<String>,
    pub path: String,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
    pub user_uuid: Uuid,
}
impl Bookmark {
    pub fn new(url: String, title: String, description: Option<String>, path: String, user_uuid: Uuid) -> Self {
        Bookmark {
            uuid: Uuid::new_v4(),
            url,
            title,
            description,
            path,
            created: UTC::now().naive_utc(),
            updated: UTC::now().naive_utc(),
            user_uuid
        }
    }
}

graphql_object!(Bookmark: Query as "Bookmark" |&self| {
    description: "Bookmark"

    field uuid() -> String as "uuid" {
        self.uuid.hyphenated().to_string()
    }

    field url() -> &String as "url" {
        &self.url
    }

    field title() -> &String as "title" {
        &self.title
    }

    field description() -> &Option<String> as "description" {
        &self.description
    }

    field path() -> &String as "path" {
        &self.path
    }

    field created() -> String as "created" {
        format!("{}", self.created)
    }

    field updated() -> String as "updated" {
        format!("{}", self.updated)
    }

    field user_uuid() -> String as "user_uuid" {
        self.user_uuid.hyphenated().to_string()
    }
});

impl<'a> From<Row<'a>> for Bookmark {
    fn from(row: Row) -> Self {
        Bookmark {
            uuid: row.get("uuid"),
            url: row.get("url"),
            title: row.get("title"),
            description: row.get("description"),
            path: row.get("path"),
            created: row.get("created"),
            updated: row.get("updated"),
            user_uuid: row.get("user_uuid"),
        }
    }
}

impl Insertable for Bookmark {
    fn insert_query(&self) -> String {
        r#"
            INSERT INTO bookmarks (uuid, title, url, description, path, created, updated, user_uuid)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8);
        "#.to_owned()
    }

    fn insert_params<'a>(&'a self) -> Box<[&'a ToSql]> {
        Box::new([&self.uuid, &self.title, &self.url, &self.description, &self.path, &self.created, &self.updated, &self.user_uuid])
    }
}

fn is_bookmark_exist(pg: &PgDatabase, url: &str, user: &User) -> Result<bool> {
    let query = "SELECT * FROM bookmarks WHERE url = $1 AND user_uuid = $2::uuid;";
    Ok(pg.exist(query, &[&url, &user.uuid])?)
}

pub fn add_bookmark_resolver<'a>(executor: &Executor<'a, Query>, bookmark: Bookmark, user: &User) -> Result<Bookmark> {
    let connection = executor.context().connection.clone().get()?;
    let pg = PgDatabase::new(connection);
    if !is_bookmark_exist(&pg, &bookmark.url, user)? {
        pg.insert(&bookmark)?;
        Ok(bookmark)
    } else {
        Err(ErrorKind::AlreadyExist.into())
    }
}

fn find_bookmarks(pg: &PgDatabase, limit: i32, offset: i32, user: &User) -> Result<Vec<Bookmark>> {
    let query = "SELECT * FROM bookmarks WHERE user_uuid = $1::uuid LIMIT $2::int OFFSET $3::int;";
    Ok(pg.find(query, &[&user.uuid, &limit, &offset])?)
}

pub fn bookmarks_resolver<'a>(executor: &Executor<'a, Query>, limit: i32, offset: i32, user: &User) -> Result<Vec<Bookmark>> {
    let connection = executor.context().connection.clone().get()?;
    let pg = PgDatabase::new(connection);
    Ok(find_bookmarks(&pg, limit, offset, user)?)
}