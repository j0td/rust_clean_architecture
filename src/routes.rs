use actix_web::{get, post, web, Error, HttpResponse};
use std::env;
use dotenv::dotenv;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use crate::actions;
use crate::domain::model::models;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[get("/user/{user_id}")]
async fn get_user(user_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    println!("GEGEGEGE");
    let user_id = user_id.into_inner();
    let conn = establish_connection();

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

#[post("/user")]
async fn add_user(form: web::Json<models::NewUser>) -> Result<HttpResponse, Error> {
    let conn = establish_connection();

    let user = web::block(move || actions::insert_new_user(&form.name, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(user))
}
