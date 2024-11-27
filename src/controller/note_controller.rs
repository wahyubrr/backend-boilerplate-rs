use crate::model::web::note::Note;
use crate::model::web::web_response::WebResponse;
use crate::service::note_service::NoteService;

use actix_web::{http::header::ContentType, web, HttpResponse};

pub struct NoteController {
    note_service: NoteService,
}

impl NoteController {
    pub fn new(note_service: NoteService) -> NoteController {
        Self { note_service }
    }

    pub fn route(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::resource("/notes")
                .route(web::get().to(find_all_notes))
                .route(web::head().to(HttpResponse::MethodNotAllowed)),
        );
        cfg.service(
            web::resource("/notes/{note_id}")
                .route(web::get().to(find_note_by_id))
                .route(web::head().to(HttpResponse::MethodNotAllowed)),
        );
    }
}

async fn find_all_notes(nc: web::Data<NoteController>) -> HttpResponse {
    let notes = nc.note_service.find_all_notes();

    let response: WebResponse<Vec<Note>>;
    match notes {
        None => {
            response = WebResponse {
                code: 404,
                status: "note not found".to_string(),
                message: "note not found".to_string(),
                data: None,
            };
        }
        Some(notes) => {
            response = WebResponse {
                code: 200,
                status: "OK".to_string(),
                message: "Success".to_string(),
                data: Some(notes),
            };
        }
    }

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&response).unwrap())
}

async fn find_note_by_id(nc: web::Data<NoteController>, path: web::Path<String>) -> HttpResponse {
    let note_id = path.into_inner();
    let note = nc.note_service.find_note_by_id(note_id);

    let response: WebResponse<Note>;
    match note {
        None => {
            response = WebResponse {
                code: 404,
                status: "note not found".to_string(),
                message: "note not found".to_string(),
                data: None,
            };
        }
        Some(note) => {
            response = WebResponse {
                code: 200,
                status: "OK".to_string(),
                message: "Success".to_string(),
                data: Some(note),
            };
        }
    }

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&response).unwrap())
}

// async fn websocket_handler(ws: WebSocketUpgrade) -> Response {
//     ws.on_upgrade(handle_websocket)
// }

// async fn handle_websocket(mut socket: WebSocket) {
//     while let Some(msg) = socket.recv().await {
//         let msg = if let Ok(msg) = msg {
//             msg
//         } else {
//             // client disconnected
//             return;
//         };

//         if socket.send(msg).await.is_err() {
//             // client disconnected
//             return;
//         }
//     }
// }
