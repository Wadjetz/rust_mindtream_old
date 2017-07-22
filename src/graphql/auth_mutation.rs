use graphql::query::Query;
use models::user::User;
use models::bookmark::Bookmark;
use models::file::File;
use models::feed_source::FeedSource;
use resolvers::feeds_sources_resolvers;
use resolvers::bookmarks_resolvers;

use std::error::Error;
use std::path::Path;

#[derive(Debug)]
pub struct AuthMutation {
    pub user: User
}

impl AuthMutation {
    pub fn new(user: User) -> Self {
        AuthMutation { user }
    }
}

graphql_object!(AuthMutation: Query as "AuthMutation" |&self| {
    description: "AuthMutation"

    field me() -> &User as "User" {
        &self.user
    }

    field create_directory(
        &executor,
        name: String as "name",
        path: String as "path",
    ) -> Result<File, String> {
        let connection = executor.context().connection.clone().get().map_err(|e| e.description().to_string())?;
        let path = Path::new(&path);
        let maybe_parent_path = path.parent();
        let directory = File::new_directory(&name, path.to_str().unwrap_or("/"), self.user.uuid);
        println!("{:?}", directory);
        Ok(directory)
    }

    field add_bookmark(
       &executor,
       url: String as "Url",
       title: String as "Title",
       path: String as "Path",
       description: Option<String> as "Description",
    ) -> Result<Bookmark, String> as "Bookmark" {
        let bookmark = Bookmark::from_request(url, title, description, path, self.user.uuid);
        bookmarks_resolvers::add_bookmark(executor, bookmark, &self.user)
            .map_err(|e| e.description().to_string())
    }

    field add_feed_source(
        &executor,
        xml_url: String as "Url",
        title: Option<String> as "Title",
    ) -> Result<FeedSource, String> as "Feed Source" {
        feeds_sources_resolvers::add_feed_source(executor, &xml_url, title)
            .map_err(|e| e.description().to_string())
    }
});