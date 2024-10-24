// Модель для комнат (текстовых или голосовых).

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Room {
    pub id: i32,
    pub name: String,
    pub is_voice_chat: bool,
}
