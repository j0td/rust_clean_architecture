use diesel::prelude::*;

use crate::models;
use diesel::mysql::MysqlConnection;

pub fn find_user_by_uid(user_id: i32, conn: &MysqlConnection) -> Result<Option<models::User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(id.eq(user_id))
        .first::<models::User>(conn)
        .optional()?;

    Ok(user)
}

pub fn insert_new_user(nm: &str, conn: &MysqlConnection) -> Result<models::User, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let new_user = models::NewUser {
        name: nm.to_owned(),
    };

    diesel::insert_into(users).values(&new_user).execute(conn)?;

    Ok(users.order(id.desc()).first(conn).unwrap())
}
