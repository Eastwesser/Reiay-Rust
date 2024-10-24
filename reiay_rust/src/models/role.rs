// Модель для хранения ролей пользователей.

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Role {
    pub id: i32,
    pub name: String,
}
