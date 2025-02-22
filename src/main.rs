use actix_web::{
    App,
    HttpServer, 
    web::Data
};

mod api;
mod model;
mod repository;
mod utils;
mod prelude;
mod error;

use repository::surrealdb_repo::SurrealDBRepo;
use api::file_api::{create_file, get_files, get_file, update_file, delete_file, hello};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let surreal = SurrealDBRepo::init().await.expect("Error connecting to SurrealDB!");
    
    let db_data = Data::new(surreal);
    
    HttpServer::new(move || { 
        App::new()
            .app_data(db_data.clone())
            .service(create_file)
            .service(get_files)
            .service(get_file)
            .service(update_file)
            .service(delete_file)
            .service(hello)
        })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}