use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

mod handlers;
mod models;

use handlers::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let student_list = web::Data::new(Mutex::<Vec<models::Student>>::new(vec![]));

    HttpServer::new(move || {
        App::new()
            .app_data(student_list.clone())
            .route("/students", web::get().to(get_students))
            .route("/students/{id}", web::get().to(get_student_by_id))
            .route("/students", web::post().to(create_student))
            .route("/students/{id}", web::put().to(update_student))
            .route("/students/{id}", web::delete().to(delete_student))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
