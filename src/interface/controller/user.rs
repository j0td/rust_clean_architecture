use actix_web::{web, Error, HttpResponse};
use std::env;
use dotenv::dotenv;
use diesel::Connection;
use crate::actions;
use crate::domain::model::models;
use diesel::mysql::MysqlConnection;

fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Connection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

// curl -X POST -H "Content-Type: application/json" http://localhost:8080/user -d '{"name": "hoge"}'
pub async fn get_user(user_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let conn = establish_connection();

    let user_id = user_id.into_inner();

    let user = web::block(move || actions::find_user_by_uid(user_id, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res = HttpResponse::NotFound()
            .body(format!("No user found with uid: {}", user_id));
        Ok(res)
    }
}

pub async fn add_user(form: web::Json<models::NewUser>) -> Result<HttpResponse, Error> {
    let conn = establish_connection();

    let user = web::block(move || actions::insert_new_user(&form.name, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(user))
}
