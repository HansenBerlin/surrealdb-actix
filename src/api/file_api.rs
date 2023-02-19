use actix_web::{
    post, get, put, delete,
    web::{Data, Json, Path},
    HttpResponse,
};

use crate::repository::{surrealdb_repo::SurrealDBRepo};
use crate::model::todo_model::{File, FileBMC, FilePatch};

#[get("/")]
pub async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("3D Files Management APIv1")
}

#[post("/files")]
pub async fn create_file(db: Data<SurrealDBRepo>, new_file: Json<File>) -> HttpResponse {
    let data = File {
        id: None,
        name: new_file.name.to_owned(),
        author: new_file.author.to_owned(),
        created: new_file.created.to_owned(),
        size: new_file.size.to_owned(),
        downloads: new_file.downloads.to_owned(),
        rating: new_file.rating.to_owned()
    };
    
    let file_detail = FileBMC::create(db, "file", data ).await;

    match file_detail {
         Ok(file) => HttpResponse::Ok().json(file),
         Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/files/{id}")]
pub async fn get_file(db: Data<SurrealDBRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    
    let file_detail = FileBMC::get(db, &id).await;
    
    match file_detail {
        Ok(file) => HttpResponse::Ok().json(file),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/files/{id}")]
pub async fn update_file(db: Data<SurrealDBRepo>, path: Path<String>, file_patch: Json<FilePatch>) -> HttpResponse {
    let id = path.into_inner();
    
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };

    let data = FilePatch {
        name: file_patch.name.to_owned(),
        author: file_patch.author.to_owned(),
        created: file_patch.created.to_owned(),
    };
    
    let update_result = FileBMC::update(db, &id, data).await;
    
    match update_result {
        Ok(file) => HttpResponse::Ok().json(file),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
#[delete("/files/{id}")]
pub async fn delete_file(db: Data<SurrealDBRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    
    let result = FileBMC::delete(db, &id).await;
    
    match result {
        Ok(file) => HttpResponse::Ok().json(file),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/files")]
pub async fn get_files(db: Data<SurrealDBRepo>) -> HttpResponse {
    let result = FileBMC::get_all(db).await;
    
    match result {
        Ok(files) => HttpResponse::Ok().json(files),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
   }
}