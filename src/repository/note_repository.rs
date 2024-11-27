pub struct NoteRepository {}

use crate::model::web::note::Note;

impl NoteRepository {
    pub fn new() -> Self {
        Self {}
    }
}

impl NoteRepository {
    pub fn find_all_notes(&self) -> Option<Vec<Note>> {
        Some(vec![Note {
            id: "1".to_string(),
            title: "Hello World".to_string(),
            content: "This is a test note".to_string(),
            access_status: 1,
            created_by: "admin".to_string(),
            created_at: "2023-01-01".to_string(),
            updated_at: "2023-01-01".to_string(),
        }])
    }

    pub fn find_note_by_id(&self, id: String) -> Option<Note> {
        if id != "1" {
            return None;
        }

        Some(Note {
            id: id.to_string(),
            title: "Hello World".to_string(),
            content: "This is a test note".to_string(),
            access_status: 1,
            created_by: "admin".to_string(),
            created_at: "2023-01-01".to_string(),
            updated_at: "2023-01-01".to_string(),
        })
    }
}
