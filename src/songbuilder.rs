use crate::notes::NoteInfo;

pub struct SongBuilder {
    pub notes: Vec<NoteInfo>,
    pub voices: Vec<VoiceInfo>,
}

pub struct VoiceInfo {
    pub first_note_index: u32,
    last_note_index: u32,
}

impl SongBuilder {
    pub fn new() -> Self {
        SongBuilder { notes: Vec::<NoteInfo>::new(), voices: Vec::<VoiceInfo>::new() }
    }

    pub fn add(&mut self, voice: u8, note: NoteInfo) {
        let new_note_index: u32 = self.notes.len() as u32;

        let new_voice: bool = voice as usize == self.voices.len();

        if new_voice {
            let new_voice = VoiceInfo {
                first_note_index: new_note_index,
                last_note_index: new_note_index,
            };

            self.voices.push(new_voice);
        }

        let last_note_index: u32 = self.voices[voice as usize].last_note_index;

        if !new_voice {
            self.notes[last_note_index as usize].next_note_index = new_note_index;
        }

        self.notes.push(NoteInfo { motor_id: voice, ..note });

        self.voices[voice as usize].last_note_index = new_note_index;
    }
}
