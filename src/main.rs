pub mod controller;
pub mod model;
pub mod repository;
pub mod service;

use crate::controller::note_controller::NoteController;
use crate::repository::note_repository::NoteRepository;
use crate::service::note_service::NoteService;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let note_repository = NoteRepository::new();
    let note_service = NoteService::new(note_repository);
    let note_controller = web::Data::new(NoteController::new(note_service));

    HttpServer::new(move || {
        App::new().app_data(note_controller.clone()).service(
            web::scope("/api").configure(NoteController::route),
            // .service(web::scope("/api").configure(scoped_config))
            // .route(
            //     "/",
            //     web::get().to(|| async { HttpResponse::Ok().body("/") }),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
