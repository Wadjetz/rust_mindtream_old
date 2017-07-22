error_chain! {
    errors {
        AlreadyExist
        NotFound
        NotInserted
        WrongCredentials
    }

    foreign_links {
        IO(::std::io::Error);
        Jwt(::jsonwebtoken::errors::Error);
        Uuid(::uuid::ParseError);
        Db(::postgres::error::Error);
        R2D2(::r2d2::GetTimeout);
        WS(::reqwest::Error);
        JSON(::serde_json::Error);
    }
}