use crate::model::web::note::Note;
use crate::repository::note_repository::NoteRepository;

pub struct NoteService {
    note_repository: NoteRepository,
}

impl NoteService {
    pub fn new(note_repository: NoteRepository) -> Self {
        Self {
            note_repository: note_repository,
        }
    }
}

impl NoteService {
    pub fn find_all_notes(&self) -> Option<Vec<Note>> {
        let notes = self.note_repository.find_all_notes();

        match notes {
            Some(notes) => Some(notes),
            None => None,
        }
    }

    pub fn find_note_by_id(&self, id: String) -> Option<Note> {
        let note = self.note_repository.find_note_by_id(id);

        match note {
            Some(note) => Some(note),
            None => None,
        }
    }
}
