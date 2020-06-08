use actix_web::{get, middleware, post, web, App, Error, HttpResponse, HttpServer};
use crate::actions;
use crate::domain::model::models;

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
